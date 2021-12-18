pub const SHADER: &str = r#"
    precision highp float;
    uniform vec4 uColor;
    uniform float uOpacity;
    void main() {
        gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
    }
"#;
