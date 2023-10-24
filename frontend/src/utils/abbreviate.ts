export function abbreviate(str: string, visibleChars = 5) {
  return str.slice(0, visibleChars) + "...." + str.slice(-visibleChars);
}
