use metashrew_support::index_pointer::KeyValuePointer;
use metashrew_support::compat::to_arraybuffer_layout;

use alkanes_runtime::{
  declare_alkane, message::MessageDispatch, storage::StoragePointer, token::Token,
  runtime::AlkaneResponder
};

use alkanes_support::{
  cellpack::Cellpack, id::AlkaneId,
  parcel::{AlkaneTransfer, AlkaneTransferParcel}, response::CallResponse
};

use anyhow::{anyhow, Result};
use std::sync::Arc;

mod svg_generator;
use svg_generator::SvgGenerator;

/// Orbital template ID / Child contract template
const ROYALTY_NFT_ORBITAL_TEMPLATE_ID: u128 = 0x378;

/// Payment configuration - Multi-token support
/// UPDATE THESE IDs FOR YOUR TARGET NETWORK

/// For regtest, deploy your wrapped tokens first and update these IDs
/// For mainnet, use the actual deployed token IDs
const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 }; // UPDATE: Deploy frBTC first
const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 };  // UPDATE: Deploy BUSD first

/// Payment amounts per token type (adjust based on token values)
const FRBTC_AMOUNT_PER_MINT: u128 = 10000; // 0.0001 BTC equivalent in satoshis
const BUSD_AMOUNT_PER_MINT: u128 = 1000000; // $10 in BUSD (assuming 6 decimals)

/// Batch minting limits
const MAX_PURCHASE_PER_TX: u128 = 3; // Maximum NFTs per transaction

/// Royalty configuration
const ROYALTY_PERCENTAGE: u128 = 500; // 5% in basis points (500/10000)
const ROYALTY_RECIPIENT: AlkaneId = AlkaneId { block: 2, tx: 0 }; // Collection contract receives royalties

/// Primary sales configuration
const PRIMARY_SALES_RECIPIENT: AlkaneId = AlkaneId { block: 2, tx: 0 }; // Where primary mint payments go

/// Supported payment tokens
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaymentToken {
    FrBTC,
    BUSD,
}

impl PaymentToken {
    fn get_id(&self) -> AlkaneId {
        match self {
            PaymentToken::FrBTC => FRBTC_TOKEN_ID,
            PaymentToken::BUSD => BUSD_TOKEN_ID,
        }
    }
    
    fn get_price_per_mint(&self) -> u128 {
        match self {
            PaymentToken::FrBTC => FRBTC_AMOUNT_PER_MINT,
            PaymentToken::BUSD => BUSD_AMOUNT_PER_MINT,
        }
    }
    
    fn from_alkane_id(id: &AlkaneId) -> Option<PaymentToken> {
        if *id == FRBTC_TOKEN_ID {
            Some(PaymentToken::FrBTC)
        } else if *id == BUSD_TOKEN_ID {
            Some(PaymentToken::BUSD)
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct RoyaltyNFTCollection(());

impl AlkaneResponder for RoyaltyNFTCollection {}

#[derive(MessageDispatch)]
enum RoyaltyNFTCollectionMessage {
  #[opcode(0)]
  Initialize,

  #[opcode(69)]
  AuthMintOrbital { count: u128 },

  #[opcode(77)]
  MintOrbital,

  #[opcode(99)]
  #[returns(String)]
  GetName,

  #[opcode(100)]
  #[returns(String)]
  GetSymbol,

  #[opcode(101)]
  #[returns(u128)]
  GetTotalSupply,

  #[opcode(102)]
  #[returns(u128)]
  GetOrbitalCount,

  #[opcode(999)]
  #[returns(String)]
  GetAttributes { index: u128 },

  #[opcode(1000)]
  #[returns(Vec<u8>)]
  GetData { index: u128 },

  #[opcode(1001)]
  #[returns(Vec<u8>)]
  GetInstanceAlkaneId { index: u128 },

  #[opcode(1002)]
  #[returns(String)]
  GetInstanceIdentifier { index: u128 },

  #[opcode(200)]
  #[returns(Vec<u8>)]
  GetRoyaltyInfo,

  #[opcode(201)]
  WithdrawFunds { token_type: u128, amount: u128 },

  #[opcode(202)]
  #[returns(Vec<u8>)]
  GetAcceptedTokens,

  #[opcode(203)]
  #[returns(Vec<u8>)]
  GetTokenPrices,
}

impl Token for RoyaltyNFTCollection {
  fn name(&self) -> String {
    return String::from("Alkane RoyaltyNFT")
  }

  fn symbol(&self) -> String {
    return String::from("alkane-royalty-nft");
  }
}

impl RoyaltyNFTCollection {
  fn initialize(&self) -> Result<CallResponse> {
    self.observe_initialization()?;
    let context = self.context()?;

    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Collection token acts as auth token for contract minting without any limits
    response.alkanes.0.push(AlkaneTransfer {
      id: context.myself.clone(),
      value: 10u128,
    });

    Ok(response)
  }

  fn auth_mint_orbital(&self, count: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Authorized mints
    self.only_owner()?;

    let mut minted_orbitals = Vec::new();

    for _ in 0..count {
      minted_orbitals.push(self.create_mint_transfer()?);
    }

    response.alkanes.0.extend(minted_orbitals);

    Ok(response)
  }

  fn mint_orbital(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    // Calculate how many NFTs can be purchased with the provided payment
    let purchase_count = self.calculate_purchase_count()?;
    
    // Mint multiple orbitals in one transaction
    let mut minted_orbitals = Vec::new();
    for _ in 0..purchase_count {
      minted_orbitals.push(self.create_mint_transfer()?);
    }

    response.alkanes.0.extend(minted_orbitals);

    Ok(response)
  }

  fn create_mint_transfer(&self) -> Result<AlkaneTransfer> {
    let index = self.instances_count();

    if index >= self.max_mints() {
      return Err(anyhow!("Alkane RoyaltyNFT have fully minted out"));
    }

    let cellpack = Cellpack {
      target: AlkaneId {
        block: 6,
        tx: ROYALTY_NFT_ORBITAL_TEMPLATE_ID,
      },
      inputs: vec![0x0, index],
    };

    let sequence = self.sequence();
    let response = self.call(&cellpack, &AlkaneTransferParcel::default(), self.fuel())?;

    let orbital_id = AlkaneId {
      block: 2,
      tx: sequence,
    };

    self.add_instance(&orbital_id)?;

    if response.alkanes.0.len() < 1 {
      Err(anyhow!("orbital token not returned with factory"))
    } else {
      Ok(response.alkanes.0[0])
    }
  }

  fn max_mints(&self) -> u128 {
    3333
  }



  fn get_name(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.name().into_bytes();

    Ok(response)
  }

  fn get_symbol(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.symbol().into_bytes();

    Ok(response)
  }

  fn get_total_supply(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = 1u128.to_le_bytes().to_vec();

    Ok(response)
  }

  fn get_orbital_count(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.instances_count().to_le_bytes().to_vec();

    Ok(response)
  }

  fn get_attributes(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let attributes = SvgGenerator::get_attributes(index)?;
    response.data = attributes.into_bytes();
    Ok(response)
  }

  fn get_data(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let svg = SvgGenerator::generate_svg(index)?;
    response.data = svg.into_bytes();
    Ok(response)
  }

  fn instances_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/instances")
  }

  fn instances_count(&self) -> u128 {
    self.instances_pointer().get_value::<u128>()
  }

  fn set_instances_count(&self, count: u128) {
    self.instances_pointer().set_value::<u128>(count);
  }

  fn add_instance(&self, instance_id: &AlkaneId) -> Result<u128> {
    let count = self.instances_count();
    let new_count = count.checked_add(1)
      .ok_or_else(|| anyhow!("instances count overflow"))?;

    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&instance_id.block.to_le_bytes());
    bytes.extend_from_slice(&instance_id.tx.to_le_bytes());

    let bytes_vec = new_count.to_le_bytes().to_vec();
    let mut instance_pointer = self.instances_pointer().select(&bytes_vec);
    instance_pointer.set(Arc::new(bytes));
    
    self.set_instances_count(new_count);
    
    Ok(new_count)
  }

  fn verify_payment(&self, required_amount: u128, payment_token: PaymentToken) -> Result<()> {
    let context = self.context()?;
    
    // Check for payment in incoming alkanes for the specific token
    let total_payment = context.incoming_alkanes.0.iter()
      .filter(|transfer| transfer.id == payment_token.get_id())
      .map(|transfer| transfer.value)
      .sum::<u128>();
    
    if total_payment < required_amount {
      return Err(anyhow!("Insufficient payment: {} units of {:?} required, {} provided", required_amount, payment_token, total_payment));
    }
    
    Ok(())
  }

  fn calculate_purchase_count(&self) -> Result<u128> {
    let context = self.context()?;
    
    let mut total_purchase_count = 0u128;
    
    // Calculate purchase count for each supported token
    for transfer in &context.incoming_alkanes.0 {
      if let Some(payment_token) = PaymentToken::from_alkane_id(&transfer.id) {
        let price_per_mint = payment_token.get_price_per_mint();
        let purchase_count_for_token = transfer.value / price_per_mint;
        total_purchase_count += purchase_count_for_token;
      }
    }
    
    if total_purchase_count == 0 {
      return Err(anyhow!("No valid payment provided. Accepted tokens: frBTC ({}), BUSD ({})", 
                         FRBTC_AMOUNT_PER_MINT, BUSD_AMOUNT_PER_MINT));
    }
    
    // Apply maximum purchase limit
    let final_count = std::cmp::min(total_purchase_count, MAX_PURCHASE_PER_TX);
    
    Ok(final_count)
  }

  fn only_owner(&self) -> Result<()> {
    let context = self.context()?;

    if context.incoming_alkanes.0.len() != 1 {
      return Err(anyhow!(
        "did not authenticate with only the collection token"
      ));
    }

    let transfer = context.incoming_alkanes.0[0].clone();
    if transfer.id != context.myself.clone() {
      return Err(anyhow!("supplied alkane is not collection token"));
    }

    if transfer.value < 1 {
      return Err(anyhow!(
        "less than 1 unit of collection token supplied to authenticate"
      ));
    }

    Ok(())
  }

  fn lookup_instance(&self, index: u128) -> Result<AlkaneId> {
    // Add 1 to index since instances are stored at 1-based indices
    let storage_index = index + 1;
    let bytes_vec = storage_index.to_le_bytes().to_vec();
    
    let instance_pointer = self.instances_pointer().select(&bytes_vec);
    
    let bytes = instance_pointer.get();
    if bytes.len() != 32 {
      return Err(anyhow!("Invalid instance data length"));
    }

    let block_bytes = &bytes[..16];
    let tx_bytes = &bytes[16..];

    let block = u128::from_le_bytes(block_bytes.try_into().unwrap());
    let tx = u128::from_le_bytes(tx_bytes.try_into().unwrap());

    Ok(AlkaneId { block, tx })
  }

  fn get_instance_alkane_id(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let instance_id = self.lookup_instance(index)?;

    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&instance_id.block.to_le_bytes());
    bytes.extend_from_slice(&instance_id.tx.to_le_bytes());

    response.data = bytes;
    Ok(response)
  }

  fn get_instance_identifier(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let instance_id = self.lookup_instance(index)?;
    let instance_str = format!("{}:{}", instance_id.block, instance_id.tx);
    
    response.data = instance_str.into_bytes();
    Ok(response)
  }

  fn get_royalty_info(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    
    // Return royalty info: [percentage, recipient_block, recipient_tx]
    let mut data = Vec::new();
    data.extend_from_slice(&ROYALTY_PERCENTAGE.to_le_bytes());
    data.extend_from_slice(&context.myself.block.to_le_bytes()); // Collection contract receives royalties
    data.extend_from_slice(&context.myself.tx.to_le_bytes());
    
    response.data = data;
    Ok(response)
  }

  /// Withdraw funds for a specific token type
  /// Parameters: [token_type (0=frBTC, 1=BUSD), amount]
  fn withdraw_funds(&self, token_type: u128, amount: u128) -> Result<CallResponse> {
    // Only the contract owner can withdraw funds
    self.only_owner()?;
    
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    
    // Verify withdrawal amount is reasonable (not zero, not excessive)
    if amount == 0 {
      return Err(anyhow!("Withdrawal amount must be greater than zero"));
    }
    
    // Determine which token to withdraw
    let payment_token = match token_type {
      0 => PaymentToken::FrBTC,
      1 => PaymentToken::BUSD,
      _ => return Err(anyhow!("Invalid token type. Use 0 for frBTC, 1 for BUSD")),
    };
    
    // Transfer the requested amount of the specified token to the caller
    response.alkanes.0.push(AlkaneTransfer {
      id: payment_token.get_id(),
      value: amount,
    });
    
    Ok(response)
  }

  fn get_accepted_tokens(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    
    // Return accepted token IDs: [frBTC_block, frBTC_tx, BUSD_block, BUSD_tx]
    let mut data = Vec::new();
    data.extend_from_slice(&FRBTC_TOKEN_ID.block.to_le_bytes());
    data.extend_from_slice(&FRBTC_TOKEN_ID.tx.to_le_bytes());
    data.extend_from_slice(&BUSD_TOKEN_ID.block.to_le_bytes());
    data.extend_from_slice(&BUSD_TOKEN_ID.tx.to_le_bytes());
    
    response.data = data;
    Ok(response)
  }

  fn get_token_prices(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    
    // Return token prices: [frBTC_price, BUSD_price]
    let mut data = Vec::new();
    data.extend_from_slice(&FRBTC_AMOUNT_PER_MINT.to_le_bytes());
    data.extend_from_slice(&BUSD_AMOUNT_PER_MINT.to_le_bytes());
    
    response.data = data;
    Ok(response)
  }
}

declare_alkane! {
  impl AlkaneResponder for RoyaltyNFTCollection {
    type Message = RoyaltyNFTCollectionMessage;
  }
}
