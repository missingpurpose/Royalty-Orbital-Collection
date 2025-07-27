use serde_json::json;
use anyhow::Result;

pub struct SvgGenerator;

impl SvgGenerator {
  /// Generate algorithmic attributes based on NFT index
  pub fn get_attributes(index: u128) -> Result<String> {
    let art_style = Self::get_art_style(index);
    let color_palette = Self::get_color_palette(index);
    let pattern_type = Self::get_pattern_type(index);
    let complexity = Self::get_complexity(index);
    let symmetry = Self::get_symmetry(index);
    let energy = Self::get_energy_level(index);

    let attributes = json!({
      "art_style": art_style,
      "color_palette": color_palette,
      "pattern_type": pattern_type,
      "complexity": complexity,
      "symmetry": symmetry,
      "energy_level": energy,
      "rarity_score": Self::calculate_rarity_score(index)
    });

    Ok(attributes.to_string())
  }

  /// Generate algorithmic SVG art based on NFT index
  pub fn generate_svg(index: u128) -> Result<String> {
    let mut svg = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str("<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 400 400\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    
    // Add gradient definitions
    svg.push_str(&Self::generate_gradients(index));
    
    // Generate background
    svg.push_str(&Self::generate_background(index));
    
    // Generate main algorithmic pattern based on art style
    match Self::get_art_style_enum(index) {
      ArtStyle::GeometricFractal => svg.push_str(&Self::generate_fractal_pattern(index)),
      ArtStyle::FlowField => svg.push_str(&Self::generate_flow_field(index)),
      ArtStyle::CirclePacking => svg.push_str(&Self::generate_circle_packing(index)),
      ArtStyle::Mandala => svg.push_str(&Self::generate_mandala(index)),
      ArtStyle::WaveInterference => svg.push_str(&Self::generate_wave_pattern(index)),
      ArtStyle::Crystalline => svg.push_str(&Self::generate_crystal_pattern(index)),
    }
    
    // Add overlay effects
    svg.push_str(&Self::generate_overlay_effects(index));
    
    // Add signature/index number
    svg.push_str(&format!("<text x=\"20\" y=\"380\" font-family=\"monospace\" font-size=\"12\" fill=\"white\" opacity=\"0.6\">#{}</text>\n", index));
    
    svg.push_str("</svg>");
    Ok(svg)
  }

  // Art Style Classification
  fn get_art_style_enum(index: u128) -> ArtStyle {
    match index % 6 {
      0 => ArtStyle::GeometricFractal,
      1 => ArtStyle::FlowField,
      2 => ArtStyle::CirclePacking,
      3 => ArtStyle::Mandala,
      4 => ArtStyle::WaveInterference,
      _ => ArtStyle::Crystalline,
    }
  }

  fn get_art_style(index: u128) -> String {
    match Self::get_art_style_enum(index) {
      ArtStyle::GeometricFractal => "Geometric Fractal".to_string(),
      ArtStyle::FlowField => "Flow Field".to_string(),
      ArtStyle::CirclePacking => "Circle Packing".to_string(),
      ArtStyle::Mandala => "Sacred Mandala".to_string(),
      ArtStyle::WaveInterference => "Wave Interference".to_string(),
      ArtStyle::Crystalline => "Crystalline Structure".to_string(),
    }
  }

  fn get_color_palette(index: u128) -> String {
    let palettes = [
      "Sunset", "Ocean", "Forest", "Aurora", "Volcanic", "Desert", 
      "Cosmic", "Neon", "Pastel", "Monochrome", "Rainbow", "Earth"
    ];
    palettes[((index / 6) % 12) as usize].to_string()
  }

  fn get_pattern_type(index: u128) -> String {
    let patterns = ["Organic", "Geometric", "Hybrid", "Chaotic", "Ordered", "Flowing"];
    patterns[((index / 72) % 6) as usize].to_string()
  }

  fn get_complexity(index: u128) -> String {
    match (index / 432) % 5 {
      0 => "Minimal".to_string(),
      1 => "Simple".to_string(),
      2 => "Moderate".to_string(),
      3 => "Complex".to_string(),
      _ => "Intricate".to_string(),
    }
  }

  fn get_symmetry(index: u128) -> String {
    let symmetries = ["Radial", "Bilateral", "Asymmetric", "Rotational"];
    symmetries[((index / 2160) % 4) as usize].to_string()
  }

  fn get_energy_level(index: u128) -> String {
    match (index / 8640) % 4 {
      0 => "Calm".to_string(),
      1 => "Balanced".to_string(),
      2 => "Dynamic".to_string(),
      _ => "Explosive".to_string(),
    }
  }

  fn calculate_rarity_score(index: u128) -> u128 {
    // Calculate rarity based on various factors
    let mut score = 0u128;
    
    // Art style rarity
    score += match Self::get_art_style_enum(index) {
      ArtStyle::Mandala => 100,
      ArtStyle::Crystalline => 90,
      ArtStyle::WaveInterference => 80,
      ArtStyle::FlowField => 70,
      ArtStyle::CirclePacking => 60,
      ArtStyle::GeometricFractal => 50,
    };
    
    // Color palette rarity
    score += match (index / 6) % 12 {
      0 | 11 => 50, // Sunset and Earth are rarer
      1 | 2 => 40,  // Ocean and Forest
      _ => 30,      // Others
    };
    
    // Complexity bonus
    score += match (index / 432) % 5 {
      4 => 30, // Intricate
      3 => 20, // Complex
      _ => 10,
    };
    
    score
  }

  // Gradient Generation
  fn generate_gradients(index: u128) -> String {
    let palette = Self::get_color_palette_colors(index);
    format!(r#"
    <defs>
      <radialGradient id="bg-gradient" cx="50%" cy="50%" r="70%">
        <stop offset="0%" style="stop-color:{};stop-opacity:0.8"/>
        <stop offset="100%" style="stop-color:{};stop-opacity:1"/>
      </radialGradient>
      <linearGradient id="pattern-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" style="stop-color:{}"/>
        <stop offset="50%" style="stop-color:{}"/>
        <stop offset="100%" style="stop-color:{}"/>
      </linearGradient>
    </defs>
    "#, palette.0, palette.1, palette.2, palette.3, palette.4)
  }

  // Background Generation
  fn generate_background(index: u128) -> String {
    format!(r#"<rect width="400" height="400" fill="url(#bg-gradient)"/>{}"#, 
      Self::generate_background_texture(index))
  }

  fn generate_background_texture(index: u128) -> String {
    let seed = index * 7919; // Large prime for distribution
    let mut texture = String::new();
    
    // Add subtle background stars/dots
    for i in 0..20 {
      let x = (seed * (i + 1) * 73) % 400;
      let y = (seed * (i + 1) * 97) % 400;
      let size = ((seed * (i + 1)) % 3) + 1;
      texture.push_str(&format!(
        r#"<circle cx="{}" cy="{}" r="{}" fill="white" opacity="0.1"/>"#,
        x, y, size
      ));
    }
    
    texture
  }

  // Color Palette Generation
  fn get_color_palette_colors(index: u128) -> (String, String, String, String, String) {
    match (index / 6) % 12 {
      0 => ("hsl(10, 80%, 60%)".to_string(), "hsl(30, 90%, 50%)".to_string(), "hsl(50, 85%, 55%)".to_string(), "hsl(20, 75%, 45%)".to_string(), "hsl(340, 70%, 50%)".to_string()), // Sunset
      1 => ("hsl(200, 80%, 40%)".to_string(), "hsl(220, 90%, 60%)".to_string(), "hsl(180, 85%, 45%)".to_string(), "hsl(240, 70%, 50%)".to_string(), "hsl(160, 75%, 40%)".to_string()), // Ocean
      2 => ("hsl(120, 60%, 30%)".to_string(), "hsl(100, 70%, 40%)".to_string(), "hsl(80, 65%, 45%)".to_string(), "hsl(140, 55%, 35%)".to_string(), "hsl(60, 60%, 50%)".to_string()), // Forest
      3 => ("hsl(300, 80%, 60%)".to_string(), "hsl(180, 90%, 50%)".to_string(), "hsl(60, 85%, 55%)".to_string(), "hsl(320, 75%, 45%)".to_string(), "hsl(200, 70%, 50%)".to_string()), // Aurora
      4 => ("hsl(0, 90%, 50%)".to_string(), "hsl(20, 95%, 60%)".to_string(), "hsl(40, 90%, 55%)".to_string(), "hsl(10, 85%, 45%)".to_string(), "hsl(350, 80%, 40%)".to_string()), // Volcanic
      5 => ("hsl(30, 70%, 50%)".to_string(), "hsl(45, 80%, 60%)".to_string(), "hsl(60, 75%, 55%)".to_string(), "hsl(20, 65%, 45%)".to_string(), "hsl(40, 70%, 40%)".to_string()), // Desert
      6 => ("hsl(270, 80%, 50%)".to_string(), "hsl(240, 90%, 60%)".to_string(), "hsl(300, 85%, 55%)".to_string(), "hsl(210, 75%, 45%)".to_string(), "hsl(330, 70%, 50%)".to_string()), // Cosmic
      7 => ("hsl(120, 100%, 50%)".to_string(), "hsl(300, 100%, 50%)".to_string(), "hsl(60, 100%, 50%)".to_string(), "hsl(180, 100%, 50%)".to_string(), "hsl(0, 100%, 50%)".to_string()), // Neon
      8 => ("hsl(300, 40%, 80%)".to_string(), "hsl(60, 50%, 85%)".to_string(), "hsl(180, 45%, 80%)".to_string(), "hsl(120, 40%, 75%)".to_string(), "hsl(30, 50%, 85%)".to_string()), // Pastel
      9 => ("hsl(0, 0%, 20%)".to_string(), "hsl(0, 0%, 60%)".to_string(), "hsl(0, 0%, 80%)".to_string(), "hsl(0, 0%, 40%)".to_string(), "hsl(0, 0%, 90%)".to_string()), // Monochrome
      10 => ("hsl(0, 80%, 50%)".to_string(), "hsl(60, 80%, 50%)".to_string(), "hsl(120, 80%, 50%)".to_string(), "hsl(240, 80%, 50%)".to_string(), "hsl(300, 80%, 50%)".to_string()), // Rainbow
      _ => ("hsl(30, 60%, 40%)".to_string(), "hsl(20, 70%, 30%)".to_string(), "hsl(40, 65%, 50%)".to_string(), "hsl(10, 55%, 35%)".to_string(), "hsl(50, 60%, 45%)".to_string()), // Earth
    }
  }

  // Pattern Generators
  fn generate_fractal_pattern(index: u128) -> String {
    let seed = index * 1931; // Prime for good distribution
    let mut pattern = String::new();
    let colors = Self::get_color_palette_colors(index);
    
    // Generate recursive squares
    for depth in 0..6 {
      let count = 4_u128.pow(depth as u32);
      let size = 200.0 / (2.0_f64.powi(depth as i32));
      
      for i in 0..count.min(20) { // Limit for performance
        let angle = (seed * (depth as u128 + 1) * (i + 1) * 41) as f64 * 0.01;
        let x = 200.0 + angle.cos() * (50.0 + depth as f64 * 20.0);
        let y = 200.0 + angle.sin() * (50.0 + depth as f64 * 20.0);
        let rotation = (seed * (i + 1) * 73) % 360;
        
        let color = match depth % 5 {
          0 => &colors.0, 1 => &colors.1, 2 => &colors.2, 3 => &colors.3, _ => &colors.4
        };
        
        pattern.push_str(&format!(
          r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" opacity="0.7" transform="rotate({} {} {})"/>"#,
          x - size/2.0, y - size/2.0, size, size, color, rotation, x, y
        ));
      }
    }
    
    pattern
  }

  fn generate_flow_field(index: u128) -> String {
    let seed = index * 2017;
    let mut pattern = String::new();
    let colors = Self::get_color_palette_colors(index);
    
    // Generate flowing curves
    for i in 0..30 {
      let start_x = ((seed * (i + 1) * 83) % 400) as f64;
      let start_y = ((seed * (i + 1) * 97) % 400) as f64;
      let color_idx = i % 5;
      let color = match color_idx {
        0 => &colors.0, 1 => &colors.1, 2 => &colors.2, 3 => &colors.3, _ => &colors.4
      };
      
      let mut path = format!("M {} {}", start_x, start_y);
      let mut x = start_x;
      let mut y = start_y;
      
      for _step in 0..20 {
        let field_x = (x / 400.0 * std::f64::consts::PI * 4.0 + seed as f64 * 0.01).sin();
        let field_y = (y / 400.0 * std::f64::consts::PI * 4.0 + seed as f64 * 0.01).cos();
        
        x += field_x * 8.0;
        y += field_y * 8.0;
        
        x = x.max(0.0).min(400.0);
        y = y.max(0.0).min(400.0);
        
        path.push_str(&format!(" L {} {}", x, y));
      }
      
      pattern.push_str(&format!(
        r#"<path d="{}" stroke="{}" stroke-width="2" fill="none" opacity="0.8"/>"#,
        path, color
      ));
    }
    
    pattern
  }

  fn generate_circle_packing(index: u128) -> String {
    let seed = index * 2099;
    let mut pattern = String::new();
    let colors = Self::get_color_palette_colors(index);
    
    // Generate packed circles
    for i in 0..50 {
      let x = ((seed * (i + 1) * 89) % 360) + 20;
      let y = ((seed * (i + 1) * 103) % 360) + 20;
      let radius = ((seed * (i + 1) * 67) % 40) + 5;
      let color_idx = i % 5;
      let color = match color_idx {
        0 => &colors.0, 1 => &colors.1, 2 => &colors.2, 3 => &colors.3, _ => &colors.4
      };
      
      pattern.push_str(&format!(
        r#"<circle cx="{}" cy="{}" r="{}" fill="{}" opacity="0.6" stroke="white" stroke-width="1"/>"#,
        x, y, radius, color
      ));
    }
    
    pattern
  }

  fn generate_mandala(index: u128) -> String {
    let _seed = index * 2111;
    let mut pattern = String::new();
    let colors = Self::get_color_palette_colors(index);
    
    // Generate mandala pattern
    let center_x = 200.0;
    let center_y = 200.0;
    
    for ring in 1..8 {
      let radius = ring as f64 * 25.0;
      let points = ring * 8;
      
      for i in 0..points {
        let angle = (i as f64 / points as f64) * 2.0 * std::f64::consts::PI;
        let x = center_x + angle.cos() * radius;
        let y = center_y + angle.sin() * radius;
        let size = 15.0 - ring as f64 * 1.5;
        
        let color_idx = (ring + i) % 5;
        let color = match color_idx {
          0 => &colors.0, 1 => &colors.1, 2 => &colors.2, 3 => &colors.3, _ => &colors.4
        };
        
        pattern.push_str(&format!(
          r#"<circle cx="{}" cy="{}" r="{}" fill="{}" opacity="0.8"/>"#,
          x, y, size, color
        ));
      }
    }
    
    pattern
  }

  fn generate_wave_pattern(index: u128) -> String {
    let seed = index * 2131;
    let mut pattern = String::new();
    let colors = Self::get_color_palette_colors(index);
    
    // Generate wave interference pattern
    for wave in 0..5 {
      let frequency = 0.02 + (wave as f64 * 0.01);
      let phase = (seed * (wave + 1) as u128) as f64 * 0.01;
      let color = match wave % 5 {
        0 => &colors.0, 1 => &colors.1, 2 => &colors.2, 3 => &colors.3, _ => &colors.4
      };
      
      let mut path = String::new();
      for x in (0..400).step_by(5) {
        let y = 200.0 + 50.0 * (x as f64 * frequency + phase).sin();
        if x == 0 {
          path.push_str(&format!("M {} {}", x, y));
        } else {
          path.push_str(&format!(" L {} {}", x, y));
        }
      }
      
      pattern.push_str(&format!(
        r#"<path d="{}" stroke="{}" stroke-width="3" fill="none" opacity="0.7"/>"#,
        path, color
      ));
    }
    
    pattern
  }

  fn generate_crystal_pattern(index: u128) -> String {
    let seed = index * 2141;
    let mut pattern = String::new();
    let colors = Self::get_color_palette_colors(index);
    
    // Generate crystalline structures
    for crystal in 0..12 {
      let center_x = ((seed * (crystal + 1) * 79) % 300) + 50;
      let center_y = ((seed * (crystal + 1) * 83) % 300) + 50;
      let sides = 3 + (crystal % 4); // 3-6 sided crystals
      let radius = 20 + ((seed * (crystal + 1) * 71) % 30);
      
      let color = match crystal % 5 {
        0 => &colors.0, 1 => &colors.1, 2 => &colors.2, 3 => &colors.3, _ => &colors.4
      };
      
      let mut points = String::new();
      for i in 0..sides {
        let angle = (i as f64 / sides as f64) * 2.0 * std::f64::consts::PI;
        let x = center_x as f64 + angle.cos() * radius as f64;
        let y = center_y as f64 + angle.sin() * radius as f64;
        
        if i == 0 {
          points.push_str(&format!("{},{}", x, y));
        } else {
          points.push_str(&format!(" {},{}", x, y));
        }
      }
      
      pattern.push_str(&format!(
        r#"<polygon points="{}" fill="{}" opacity="0.6" stroke="white" stroke-width="1"/>"#,
        points, color
      ));
    }
    
    pattern
  }

  fn generate_overlay_effects(index: u128) -> String {
    let seed = index * 2153;
    let mut overlay = String::new();
    
    // Add some sparkle effects
    for i in 0..10 {
      let x = ((seed * (i + 1) * 91) % 400) as f64;
      let y = ((seed * (i + 1) * 101) % 400) as f64;
      let size = ((seed * (i + 1) * 61) % 3) + 1;
      
      overlay.push_str(&format!(
        r#"<circle cx="{}" cy="{}" r="{}" fill="white" opacity="0.8">
          <animate attributeName="opacity" values="0.8;0.2;0.8" dur="2s" repeatCount="indefinite"/>
        </circle>"#,
        x, y, size
      ));
    }
    
    overlay
  }
}

#[derive(Debug, Clone)]
enum ArtStyle {
  GeometricFractal,
  FlowField,
  CirclePacking,
  Mandala,
  WaveInterference,
  Crystalline,
} 