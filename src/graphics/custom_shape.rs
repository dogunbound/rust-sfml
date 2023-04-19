use {
    crate::{
        ffi::{graphics as ffi, sfVector2f},
        graphics::{
            Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture,
            Transform, Transformable,
        },
        system::Vector2f,
    },
    std::{marker::PhantomData, os::raw::c_void, ptr},
};

/// The points of a custom shape.
pub trait CustomShapePoints {
    /// Gets the total count of points.
    ///
    /// Return the points count
    fn point_count(&self) -> usize;

    /// Gets a given point.
    ///
    /// # Argument
    /// * point - The index of the point to return
    ///
    /// Returns a [`Vector2f`] containing the coordinates.
    ///
    /// [`Vector2f`]: crate::system::Vector2f
    fn point(&self, point: usize) -> Vector2f;
}

/// A custom textured shape with outline.
#[derive(Debug)]
pub struct CustomShape<'s> {
    shape: *mut ffi::sfShape,
    texture: PhantomData<&'s Texture>,
    points: *mut Box<dyn CustomShapePoints + Send>,
}

unsafe extern "C" fn get_point_count_callback(obj: *mut c_void) -> usize {
    let shape = obj as *const Box<dyn CustomShapePoints + Send>;
    (*shape).point_count()
}

unsafe extern "C" fn get_point_callback(point: usize, obj: *mut c_void) -> sfVector2f {
    let shape = obj as *const Box<dyn CustomShapePoints + Send>;
    (*shape).point(point)
}

impl<'s> CustomShape<'s> {
    /// Create a new `CustomShape`
    ///
    /// # Arguments
    /// * points - Implementation of [`CustomShapePoints`]
    #[must_use]
    pub fn new(points: Box<dyn CustomShapePoints + Send>) -> CustomShape<'s> {
        let raw_impl = Box::into_raw(Box::new(points));
        let sp = unsafe {
            ffi::sfShape_create(
                Some(get_point_count_callback),
                Some(get_point_callback),
                raw_impl as *mut _,
            )
        };
        assert!(!sp.is_null(), "Failed to create CustomShape");
        CustomShape {
            shape: sp,
            texture: PhantomData,
            points: raw_impl,
        }
    }

    /// Create a new `CustomShape` with a texture
    ///
    /// # Arguments
    /// * points - Implementation of [`CustomShapePoints`] trait
    /// * texture - The texture to bind to the `CustomShape`
    #[must_use]
    pub fn with_texture(
        points: Box<dyn CustomShapePoints + Send>,
        texture: &'s Texture,
    ) -> CustomShape<'s> {
        let mut shape = Self::new(points);
        shape.set_texture(texture, true);
        shape
    }

    /// Recompute the internal geometry of a shape
    ///
    /// This function must be called by specialized shape objects
    /// everytime their points change (ie. the result of either
    /// the [`point_count`] or [`point`] callbacks is different).
    ///
    /// [`point_count`]: CustomShapePoints::point_count
    /// [`point`]: CustomShapePoints::point
    pub fn update(&mut self) {
        unsafe { ffi::sfShape_update(self.shape) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfShape {
        self.shape
    }
}

impl<'s> Shape<'s> for CustomShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfShape_setTexture(self.shape, texture, reset_rect) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfShape_setTexture(self.shape, ptr::null_mut(), true) }
    }
    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfShape_setTextureRect(self.shape, rect) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfShape_setFillColor(self.shape, color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfShape_setOutlineColor(self.shape, color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfShape_setOutlineThickness(self.shape, thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe { ffi::sfShape_getTexture(self.shape).as_ref() }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfShape_getTextureRect(self.shape) }
    }
    fn fill_color(&self) -> Color {
        unsafe { ffi::sfShape_getFillColor(self.shape) }
    }
    fn outline_color(&self) -> Color {
        unsafe { ffi::sfShape_getOutlineColor(self.shape) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfShape_getOutlineThickness(self.shape) }
    }
    fn point_count(&self) -> usize {
        unsafe { ffi::sfShape_getPointCount(self.shape) }
    }
    fn point(&self, index: usize) -> Vector2f {
        unsafe { ffi::sfShape_getPoint(self.shape, index) }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfShape_getLocalBounds(self.shape) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfShape_getGlobalBounds(self.shape) }
    }
}

impl<'s> Drawable for CustomShape<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_shape(self, states)
    }
}

impl<'s> Transformable for CustomShape<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfShape_setPosition(self.shape, position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfShape_setRotation(self.shape, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfShape_setScale(self.shape, scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfShape_setOrigin(self.shape, origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfShape_getPosition(self.shape) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfShape_getRotation(self.shape) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfShape_getScale(self.shape) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfShape_getOrigin(self.shape) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfShape_move(self.shape, offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfShape_rotate(self.shape, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfShape_scale(self.shape, factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfShape_getTransform(self.shape) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfShape_getInverseTransform(self.shape) }
    }
}

impl<'s> Drop for CustomShape<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfShape_destroy(self.shape);
            let _ = Box::from_raw(self.points);
        }
    }
}
