initSidebarItems({"constant":[["NONE_PIXBUF_ANIMATION",""],["NONE_PIXBUF_LOADER",""]],"enum":[["Colorspace","This enumeration defines the color spaces that are supported by the gdk-pixbuf library. Currently only RGB is supported."],["InterpType","This enumeration describes the different interpolation modes that can be used with the scaling functions. `InterpType::Nearest` is the fastest scaling method, but has horrible quality when scaling down. `InterpType::Bilinear` is the best choice if you aren’t sure what to choose, it has a good speed/quality balance."],["PixbufAlphaMode","These values can be passed to `gdk_pixbuf_xlib_render_to_drawable_alpha` to control how the alpha channel of an image should be handled. This function can create a bilevel clipping mask (black and white) and use it while painting the image. In the future, when the X Window System gets an alpha channel extension, it will be possible to do full alpha compositing onto arbitrary drawables. For now both cases fall back to a bilevel clipping mask."],["PixbufError","An error code in the `GDK_PIXBUF_ERROR` domain. Many gdk-pixbuf operations can cause errors in this domain, or in the `G_FILE_ERROR` domain."],["PixbufRotation","The possible rotations which can be passed to `Pixbuf::rotate_simple`. To make them easier to use, their numerical values are the actual degrees."]],"mod":[["prelude","Traits inteded for blanket imports."]],"struct":[["Pixbuf","This is the main structure in the gdk-pixbuf library. It is used to represent images. It contains information about the image’s pixel data, its color space, bits per sample, width and height, and the rowstride (the number of bytes between the start of one row and the start of the next)."],["PixbufAnimation","An opaque struct representing an animation."],["PixbufAnimationIter","An opaque struct representing an iterator which points to a certain position in an animation."],["PixbufFormat",""],["PixbufLoader","The `PixbufLoader` struct contains only private fields."],["PixbufSimpleAnim","An opaque struct representing a simple animation."]],"trait":[["PixbufAnimationExt","Trait containing all `PixbufAnimation` methods."],["PixbufLoaderExt","Trait containing all `PixbufLoader` methods."]]});