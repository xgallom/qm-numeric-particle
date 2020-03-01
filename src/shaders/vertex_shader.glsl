attribute vec4 position;
attribute vec4 color;

uniform mat4 model_view;
uniform mat4 projection;

varying lowp vec4 v_color;

void main() {
    gl_Position = projection * model_view * position;
    v_color = color;
}
