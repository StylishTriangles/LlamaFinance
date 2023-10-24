import { presetDaisy } from "unocss-preset-daisy";
import { presetAttributify, presetIcons, presetUno, transformerDirectives, transformerVariantGroup } from "unocss";
import { defineConfig } from "unocss/vite";
import presetEase from "unocss-preset-ease";

export default defineConfig({
  presets: [
    presetUno(),
    presetAttributify(),
    presetEase(),
    presetIcons({
      scale: 1.5,
      warn: true,
    }),
    presetDaisy(),
  ],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
});
