// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    pub struct AccelFlags: u32 {
        const VISIBLE = 1;
        const LOCKED = 2;
        const MASK = 7;
    }
}

impl fmt::Display for AccelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for AccelFlags {
    type GlibType = ffi::GtkAccelFlags;

    fn into_glib(self) -> ffi::GtkAccelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkAccelFlags> for AccelFlags {
    unsafe fn from_glib(value: ffi::GtkAccelFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AccelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_accel_flags_get_type()) }
    }
}

impl glib::value::ValueType for AccelFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AccelFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AccelFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct ApplicationInhibitFlags: u32 {
        const LOGOUT = 1;
        const SWITCH = 2;
        const SUSPEND = 4;
        const IDLE = 8;
    }
}

impl fmt::Display for ApplicationInhibitFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ApplicationInhibitFlags {
    type GlibType = ffi::GtkApplicationInhibitFlags;

    fn into_glib(self) -> ffi::GtkApplicationInhibitFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkApplicationInhibitFlags> for ApplicationInhibitFlags {
    unsafe fn from_glib(value: ffi::GtkApplicationInhibitFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ApplicationInhibitFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_application_inhibit_flags_get_type()) }
    }
}

impl glib::value::ValueType for ApplicationInhibitFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ApplicationInhibitFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ApplicationInhibitFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct CalendarDisplayOptions: u32 {
        const SHOW_HEADING = 1;
        const SHOW_DAY_NAMES = 2;
        const NO_MONTH_CHANGE = 4;
        const SHOW_WEEK_NUMBERS = 8;
        const SHOW_DETAILS = 32;
    }
}

impl fmt::Display for CalendarDisplayOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for CalendarDisplayOptions {
    type GlibType = ffi::GtkCalendarDisplayOptions;

    fn into_glib(self) -> ffi::GtkCalendarDisplayOptions {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkCalendarDisplayOptions> for CalendarDisplayOptions {
    unsafe fn from_glib(value: ffi::GtkCalendarDisplayOptions) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for CalendarDisplayOptions {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_calendar_display_options_get_type()) }
    }
}

impl glib::value::ValueType for CalendarDisplayOptions {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for CalendarDisplayOptions {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for CalendarDisplayOptions {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct CellRendererState: u32 {
        const SELECTED = 1;
        const PRELIT = 2;
        const INSENSITIVE = 4;
        const SORTED = 8;
        const FOCUSED = 16;
        const EXPANDABLE = 32;
        const EXPANDED = 64;
    }
}

impl fmt::Display for CellRendererState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for CellRendererState {
    type GlibType = ffi::GtkCellRendererState;

    fn into_glib(self) -> ffi::GtkCellRendererState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkCellRendererState> for CellRendererState {
    unsafe fn from_glib(value: ffi::GtkCellRendererState) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for CellRendererState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_cell_renderer_state_get_type()) }
    }
}

impl glib::value::ValueType for CellRendererState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for CellRendererState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for CellRendererState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct DestDefaults: u32 {
        const MOTION = 1;
        const HIGHLIGHT = 2;
        const DROP = 4;
        const ALL = 7;
    }
}

impl fmt::Display for DestDefaults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DestDefaults {
    type GlibType = ffi::GtkDestDefaults;

    fn into_glib(self) -> ffi::GtkDestDefaults {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDestDefaults> for DestDefaults {
    unsafe fn from_glib(value: ffi::GtkDestDefaults) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DestDefaults {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_dest_defaults_get_type()) }
    }
}

impl glib::value::ValueType for DestDefaults {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DestDefaults {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DestDefaults {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct DialogFlags: u32 {
        const MODAL = 1;
        const DESTROY_WITH_PARENT = 2;
        const USE_HEADER_BAR = 4;
    }
}

impl fmt::Display for DialogFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DialogFlags {
    type GlibType = ffi::GtkDialogFlags;

    fn into_glib(self) -> ffi::GtkDialogFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDialogFlags> for DialogFlags {
    unsafe fn from_glib(value: ffi::GtkDialogFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DialogFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_dialog_flags_get_type()) }
    }
}

impl glib::value::ValueType for DialogFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DialogFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DialogFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
bitflags! {
    pub struct EventControllerScrollFlags: u32 {
        const NONE = 0;
        const VERTICAL = 1;
        const HORIZONTAL = 2;
        const DISCRETE = 4;
        const KINETIC = 8;
        const BOTH_AXES = 3;
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl fmt::Display for EventControllerScrollFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
#[doc(hidden)]
impl IntoGlib for EventControllerScrollFlags {
    type GlibType = ffi::GtkEventControllerScrollFlags;

    fn into_glib(self) -> ffi::GtkEventControllerScrollFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
#[doc(hidden)]
impl FromGlib<ffi::GtkEventControllerScrollFlags> for EventControllerScrollFlags {
    unsafe fn from_glib(value: ffi::GtkEventControllerScrollFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl StaticType for EventControllerScrollFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_event_controller_scroll_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl glib::value::ValueType for EventControllerScrollFlags {
    type Type = Self;
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
unsafe impl<'a> FromValue<'a> for EventControllerScrollFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl ToValue for EventControllerScrollFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct FileFilterFlags: u32 {
        const FILENAME = 1;
        const URI = 2;
        const DISPLAY_NAME = 4;
        const MIME_TYPE = 8;
    }
}

impl fmt::Display for FileFilterFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FileFilterFlags {
    type GlibType = ffi::GtkFileFilterFlags;

    fn into_glib(self) -> ffi::GtkFileFilterFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkFileFilterFlags> for FileFilterFlags {
    unsafe fn from_glib(value: ffi::GtkFileFilterFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FileFilterFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_file_filter_flags_get_type()) }
    }
}

impl glib::value::ValueType for FileFilterFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FileFilterFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FileFilterFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
bitflags! {
    pub struct FontChooserLevel: u32 {
        const FAMILY = 0;
        const STYLE = 1;
        const SIZE = 2;
        const VARIATIONS = 4;
        const FEATURES = 8;
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl fmt::Display for FontChooserLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
#[doc(hidden)]
impl IntoGlib for FontChooserLevel {
    type GlibType = ffi::GtkFontChooserLevel;

    fn into_glib(self) -> ffi::GtkFontChooserLevel {
        self.bits()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
#[doc(hidden)]
impl FromGlib<ffi::GtkFontChooserLevel> for FontChooserLevel {
    unsafe fn from_glib(value: ffi::GtkFontChooserLevel) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl StaticType for FontChooserLevel {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_font_chooser_level_get_type()) }
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl glib::value::ValueType for FontChooserLevel {
    type Type = Self;
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
unsafe impl<'a> FromValue<'a> for FontChooserLevel {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
impl ToValue for FontChooserLevel {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct IconLookupFlags: u32 {
        const NO_SVG = 1;
        const FORCE_SVG = 2;
        const USE_BUILTIN = 4;
        const GENERIC_FALLBACK = 8;
        const FORCE_SIZE = 16;
        const FORCE_REGULAR = 32;
        const FORCE_SYMBOLIC = 64;
        const DIR_LTR = 128;
        const DIR_RTL = 256;
    }
}

impl fmt::Display for IconLookupFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for IconLookupFlags {
    type GlibType = ffi::GtkIconLookupFlags;

    fn into_glib(self) -> ffi::GtkIconLookupFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkIconLookupFlags> for IconLookupFlags {
    unsafe fn from_glib(value: ffi::GtkIconLookupFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for IconLookupFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_icon_lookup_flags_get_type()) }
    }
}

impl glib::value::ValueType for IconLookupFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IconLookupFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for IconLookupFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct InputHints: u32 {
        const NONE = 0;
        const SPELLCHECK = 1;
        const NO_SPELLCHECK = 2;
        const WORD_COMPLETION = 4;
        const LOWERCASE = 8;
        const UPPERCASE_CHARS = 16;
        const UPPERCASE_WORDS = 32;
        const UPPERCASE_SENTENCES = 64;
        const INHIBIT_OSK = 128;
        const VERTICAL_WRITING = 256;
        #[cfg(any(feature = "v3_22_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22_20")))]
        const EMOJI = 512;
        #[cfg(any(feature = "v3_22_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22_20")))]
        const NO_EMOJI = 1024;
    }
}

impl fmt::Display for InputHints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for InputHints {
    type GlibType = ffi::GtkInputHints;

    fn into_glib(self) -> ffi::GtkInputHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkInputHints> for InputHints {
    unsafe fn from_glib(value: ffi::GtkInputHints) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InputHints {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_input_hints_get_type()) }
    }
}

impl glib::value::ValueType for InputHints {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InputHints {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InputHints {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct JunctionSides: u32 {
        const NONE = 0;
        const CORNER_TOPLEFT = 1;
        const CORNER_TOPRIGHT = 2;
        const CORNER_BOTTOMLEFT = 4;
        const CORNER_BOTTOMRIGHT = 8;
        const TOP = 3;
        const BOTTOM = 12;
        const LEFT = 5;
        const RIGHT = 10;
    }
}

impl fmt::Display for JunctionSides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for JunctionSides {
    type GlibType = ffi::GtkJunctionSides;

    fn into_glib(self) -> ffi::GtkJunctionSides {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkJunctionSides> for JunctionSides {
    unsafe fn from_glib(value: ffi::GtkJunctionSides) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for JunctionSides {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_junction_sides_get_type()) }
    }
}

impl glib::value::ValueType for JunctionSides {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for JunctionSides {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for JunctionSides {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct PlacesOpenFlags: u32 {
        const NORMAL = 1;
        const NEW_TAB = 2;
        const NEW_WINDOW = 4;
    }
}

impl fmt::Display for PlacesOpenFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PlacesOpenFlags {
    type GlibType = ffi::GtkPlacesOpenFlags;

    fn into_glib(self) -> ffi::GtkPlacesOpenFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkPlacesOpenFlags> for PlacesOpenFlags {
    unsafe fn from_glib(value: ffi::GtkPlacesOpenFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PlacesOpenFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_places_open_flags_get_type()) }
    }
}

impl glib::value::ValueType for PlacesOpenFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlacesOpenFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PlacesOpenFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct RecentFilterFlags: u32 {
        const URI = 1;
        const DISPLAY_NAME = 2;
        const MIME_TYPE = 4;
        const APPLICATION = 8;
        const GROUP = 16;
        const AGE = 32;
    }
}

impl fmt::Display for RecentFilterFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for RecentFilterFlags {
    type GlibType = ffi::GtkRecentFilterFlags;

    fn into_glib(self) -> ffi::GtkRecentFilterFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkRecentFilterFlags> for RecentFilterFlags {
    unsafe fn from_glib(value: ffi::GtkRecentFilterFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RecentFilterFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_recent_filter_flags_get_type()) }
    }
}

impl glib::value::ValueType for RecentFilterFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RecentFilterFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RecentFilterFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct RegionFlags: u32 {
        const EVEN = 1;
        const ODD = 2;
        const FIRST = 4;
        const LAST = 8;
        const ONLY = 16;
        const SORTED = 32;
    }
}

impl fmt::Display for RegionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for RegionFlags {
    type GlibType = ffi::GtkRegionFlags;

    fn into_glib(self) -> ffi::GtkRegionFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkRegionFlags> for RegionFlags {
    unsafe fn from_glib(value: ffi::GtkRegionFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RegionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_region_flags_get_type()) }
    }
}

impl glib::value::ValueType for RegionFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RegionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RegionFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct StateFlags: u32 {
        const NORMAL = 0;
        const ACTIVE = 1;
        const PRELIGHT = 2;
        const SELECTED = 4;
        const INSENSITIVE = 8;
        const INCONSISTENT = 16;
        const FOCUSED = 32;
        const BACKDROP = 64;
        const DIR_LTR = 128;
        const DIR_RTL = 256;
        const LINK = 512;
        const VISITED = 1024;
        const CHECKED = 2048;
        const DROP_ACTIVE = 4096;
    }
}

impl fmt::Display for StateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for StateFlags {
    type GlibType = ffi::GtkStateFlags;

    fn into_glib(self) -> ffi::GtkStateFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStateFlags> for StateFlags {
    unsafe fn from_glib(value: ffi::GtkStateFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for StateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_state_flags_get_type()) }
    }
}

impl glib::value::ValueType for StateFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for StateFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for StateFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
bitflags! {
    pub struct StyleContextPrintFlags: u32 {
        const NONE = 0;
        const RECURSE = 1;
        const SHOW_STYLE = 2;
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl fmt::Display for StyleContextPrintFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
#[doc(hidden)]
impl IntoGlib for StyleContextPrintFlags {
    type GlibType = ffi::GtkStyleContextPrintFlags;

    fn into_glib(self) -> ffi::GtkStyleContextPrintFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GtkStyleContextPrintFlags> for StyleContextPrintFlags {
    unsafe fn from_glib(value: ffi::GtkStyleContextPrintFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl StaticType for StyleContextPrintFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_style_context_print_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl glib::value::ValueType for StyleContextPrintFlags {
    type Type = Self;
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
unsafe impl<'a> FromValue<'a> for StyleContextPrintFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl ToValue for StyleContextPrintFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct TargetFlags: u32 {
        const SAME_APP = 1;
        const SAME_WIDGET = 2;
        const OTHER_APP = 4;
        const OTHER_WIDGET = 8;
    }
}

impl fmt::Display for TargetFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TargetFlags {
    type GlibType = ffi::GtkTargetFlags;

    fn into_glib(self) -> ffi::GtkTargetFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTargetFlags> for TargetFlags {
    unsafe fn from_glib(value: ffi::GtkTargetFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TargetFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_target_flags_get_type()) }
    }
}

impl glib::value::ValueType for TargetFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TargetFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TargetFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct TextSearchFlags: u32 {
        const VISIBLE_ONLY = 1;
        const TEXT_ONLY = 2;
        const CASE_INSENSITIVE = 4;
    }
}

impl fmt::Display for TextSearchFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TextSearchFlags {
    type GlibType = ffi::GtkTextSearchFlags;

    fn into_glib(self) -> ffi::GtkTextSearchFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTextSearchFlags> for TextSearchFlags {
    unsafe fn from_glib(value: ffi::GtkTextSearchFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TextSearchFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_text_search_flags_get_type()) }
    }
}

impl glib::value::ValueType for TextSearchFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TextSearchFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TextSearchFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct ToolPaletteDragTargets: u32 {
        const ITEMS = 1;
        const GROUPS = 2;
    }
}

impl fmt::Display for ToolPaletteDragTargets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ToolPaletteDragTargets {
    type GlibType = ffi::GtkToolPaletteDragTargets;

    fn into_glib(self) -> ffi::GtkToolPaletteDragTargets {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkToolPaletteDragTargets> for ToolPaletteDragTargets {
    unsafe fn from_glib(value: ffi::GtkToolPaletteDragTargets) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ToolPaletteDragTargets {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_tool_palette_drag_targets_get_type()) }
    }
}

impl glib::value::ValueType for ToolPaletteDragTargets {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ToolPaletteDragTargets {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ToolPaletteDragTargets {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct TreeModelFlags: u32 {
        const ITERS_PERSIST = 1;
        const LIST_ONLY = 2;
    }
}

impl fmt::Display for TreeModelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TreeModelFlags {
    type GlibType = ffi::GtkTreeModelFlags;

    fn into_glib(self) -> ffi::GtkTreeModelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTreeModelFlags> for TreeModelFlags {
    unsafe fn from_glib(value: ffi::GtkTreeModelFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TreeModelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_tree_model_flags_get_type()) }
    }
}

impl glib::value::ValueType for TreeModelFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TreeModelFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TreeModelFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
