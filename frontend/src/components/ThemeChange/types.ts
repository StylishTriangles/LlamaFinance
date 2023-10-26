// import type { BasicColorSchema } from "@vueuse/core";

export declare type CustomTheme =
  | "night"
  | "bumblebee"
  | "synthwave"
  | "cyberpunk"
  | "garden"
  | "dracula"
  | "winter";
export interface ThemeList {
  name: string;
  id: CustomTheme;
}
