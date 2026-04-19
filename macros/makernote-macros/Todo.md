1. FromPrimitive
2. BitFlagsStruct
3. ExifStruct
<!-- #[derive(ExifStruct)] -->
---

# 🧩 1. `#[derive(FromPrimitive)]`

## 💡 Idea

Auto-generate:

* `impl From<u16>`
* `Unknown(value)` fallback

## ✨ Usage

```rust
#[derive(FromPrimitive)]
pub enum ShutterMode {
    #[value = 0] Mechanical,
    #[value = 16] Electronic,
    #[value = 48] ElectronicFrontCurtain,
    #[value = 64] ElectronicMovie,
    #[value = 80] AutoMechanical,
    #[value = 81] AutoElectronicFrontCurtain,
    #[value = 96] ElectronicHighSpeed,
    #[unknown]
    Unknown(u16),
}
```

## 🔥 Generates

```rust
impl From<u16> for ShutterMode {
    fn from(v: u16) -> Self {
        match v {
            0 => Self::Mechanical,
            16 => Self::Electronic,
            ...
            _ => Self::Unknown(v),
        }
    }
}
```

---

# 🧩 2. `#[derive(DisplayName)]`

## 💡 Idea

Turn enum variants into human-readable strings.

## ✨ Usage

```rust
#[derive(DisplayName)]
pub enum VignetteControl {
    Off,
    Low,
    Normal,
    High,
}
```

## 🔥 Generates

```rust
impl std::fmt::Display for VignetteControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Low => write!(f, "Low"),
            Self::Normal => write!(f, "Normal"),
            Self::High => write!(f, "High"),
        }
    }
}
```

(Optionally split `CamelCase → "Camel Case"`)

---

# 🧩 3. `#[derive(BitFlagsStruct)]`

## 💡 Idea

Generate struct + bit decoding from a single enum-like definition.

## ✨ Usage

```rust
#[derive(BitFlagsStruct)]
pub struct LensType(u8) {
    0 => mf,
    1 => d,
    2 => g,
    3 => vr,
    4 => reserved,
    5 => ft1,
    6 => e,
    7 => af_p,
}
```

## 🔥 Generates

```rust
pub struct LensType {
    pub mf: bool,
    pub d: bool,
    ...
}

impl From<u8> for LensType { ... }
```

---

# 🧩 4. `#[derive(TiffTag)]`

## 💡 Idea (VERY powerful)

Define tag parsing directly on enum.

## ✨ Usage

```rust
#[derive(TiffTag)]
pub enum NikonTag {
    #[tag = 0x0002, type = "u16[2]"]
    Iso([u16; 2]),

    #[tag = 0x0003, type = "string"]
    ColorMode(String),

    #[tag = 0x001b, type = "u16[7]", map = "CropHighSpeed"]
    CropHiSpeed([CropHighSpeed; 7]),
}
```

## 🔥 Generates

* Tag match logic
* Reading logic
* Conversion into struct fields

This basically replaces your entire `match tag { ... }`

---

# 🧩 5. `#[derive(ExifStruct)]` (big one)

## 💡 Idea

Auto-generate full parser from struct definition.

## ✨ Usage

```rust
#[derive(ExifStruct)]
pub struct NikonMakerNotes {
    #[tag = 0x0002]
    pub iso: Option<[u16; 2]>,

    #[tag = 0x0003]
    pub color_mode: Option<String>,

    #[tag = 0x001b, map = "CropHighSpeed"]
    pub crop_hi_speed: Option<[CropHighSpeed; 7]>,
}
```

## 🔥 Generates

* Full `parse()` function
* Tag dispatch
* Inline vs offset handling
* Type decoding

---

# 🧩 6. `#[derive(EnumFromStr)]`

## 💡 Idea

Convert EXIF string values → enums

## ✨ Usage

```rust
#[derive(EnumFromStr)]
pub enum WhiteBalance {
    Auto,
    Daylight,
    Cloudy,
    Tungsten,
}
```

---

# 🧩 7. `#[derive(RationalOps)]`

## 💡 Idea

Make your `Rational` type usable

## 🔥 Generates

```rust
impl Rational {
    pub fn to_f64(self) -> f64 { ... }
}

impl std::fmt::Display for Rational { ... }
```

---

# 🧩 8. `#[derive(TagBundle)]`

## 💡 Idea

Group related tags into sub-structs

```rust
#[derive(TagBundle)]
pub struct ExposureInfo {
    #[tag = 0x0019]
    pub bracket: Rational,

    #[tag = 0x0012]
    pub flash_comp: Rational,
}
```

---

# 🧩 9. `#[derive(EnumMetadata)]`

## 💡 Idea

Attach extra metadata like descriptions

```rust
#[derive(EnumMetadata)]
pub enum CropHighSpeed {
    #[desc = "1.3x Crop"]
    Crop1_3x,
}
```

---

# 🧠 Best combo for your project

If you build just these 4 macros, you eliminate ~80% of your boilerplate:

1. ✅ `FromPrimitive`
2. ✅ `DisplayName`
3. ✅ `BitFlagsStruct`
4. ✅ `ExifStruct` ← biggest payoff

---

# ⚡ Reality check

* `FromPrimitive` → easy
* `DisplayName` → easy
* `BitFlagsStruct` → medium
* `ExifStruct` → **hard (but insanely powerful)**

---

# 👍 If you want next step

I can help you implement one of these from scratch using only:

* `proc_macro`
* no `syn`
* no `quote`

Best starting point is **`FromPrimitive`** — small but very useful.

Just tell me which one 👍
