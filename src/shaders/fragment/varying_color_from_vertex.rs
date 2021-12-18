pub const SHADER: &str = r#"
    precision highp float;
    uniform float uOpacity;
    varying lowp vec4 vColor;
    void main() {
        gl_FragColor = vec4(vColor.r, vColor.g, vColor.b, vColor.a * uOpacity);
    }
"#;