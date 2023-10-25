import type { BasicColorSchema } from "@vueuse/core";

export declare type CustomTheme = "bumblebee" | "synthwave"
| "cyberpunk" | "garden" | "dracula" | "night" | "winter" | BasicColorSchema;
export interface ThemeList {
  name: string;
  id: CustomTheme;
}
