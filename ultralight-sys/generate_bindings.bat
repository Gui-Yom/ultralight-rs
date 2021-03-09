bindgen wrapper.h ^
-o src/bindings.rs ^
--impl-debug ^
--impl-partialeq ^
--whitelist-var "^UL.*|JS.*|ul.*|WK.*" ^
--whitelist-type "^UL.*|JS.*|ul.*|WK.*" ^
--whitelist-function "^UL.*|JS.*|ul.*|WK.*" ^
--default-enum-style rust ^
--bitfield-enum "ULWindowFlags|JSPropertyAttributes" ^
-- -IC:/apps/ultralight/include -fretain-comments-from-system-headers
