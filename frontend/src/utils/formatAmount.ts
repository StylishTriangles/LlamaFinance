interface formatAssetAmountOptions {
  minDigits?: number;
  maxDigits?: number;
}

export function formatAssetAmount(
  value: string | number | null,
  options = {} as formatAssetAmountOptions,
) {
  const amount = Number(value);
  if (
    (amount <= 0 && !options.maxDigits)
    || Number.isNaN(amount)
    || (options.maxDigits && Number(Number(value).toFixed(options.maxDigits)))
      === 0
  )
    return "0.00";
  else if (Math.abs(amount) >= 0.01 && !options.minDigits)
    return amount.toLocaleString("en-US", {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });
  else {
    let maxDigits;
    if (options.maxDigits)
      maxDigits = options.maxDigits;
    else if (options.minDigits)
      maxDigits = options.minDigits;
    else {
      const whole = Math.floor(amount);
      const fraction
        = amount >= 1 ? "" : (amount - whole).toString().substring(2);
      if (!fraction)
        maxDigits = 2;
      else {
        const firstSignificant
          = fraction.split("").findIndex(v => v !== "0") + 1;
        maxDigits = 2 * Math.round(firstSignificant / 2); // round to the nearest even number
      }
    }

    return cutTrailingZeros(
      amount.toLocaleString("en-US", {
        minimumFractionDigits: options.minDigits || 2,
        maximumFractionDigits: maxDigits,
      }),
    );
  }
}

/**
 * Formats value to:
 * - 2 decimal places if value >= 1,
 * - 4 significant digits otherwise (no more than 8 decimal places)
 */
export function formatUSDAmount(value: string | number | null) {
  value = Number(value) || 0;

  if (value >= 1)
    return "$" + value.toLocaleString("en-US", {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });

  const whole = Math.floor(value);
  const fraction = value - whole;
  const fractionFormatted = fraction.toLocaleString("en-US", {
    maximumSignificantDigits: 4,
  });

  let result;
  if (fractionFormatted.includes("."))
    result = `${whole.toLocaleString("en-US")}.${fractionFormatted
      .substring(2)
      .padEnd(2, "0")
      .substring(0, 8)}`;
  else
    result = `${(whole + Number(fraction)).toLocaleString("en-US")}.00`;

  if (Number(result) === 0)
    return "$0.00";
  return "$" + result;
}

function cutTrailingZeros(amount: string) {
  const cutAmount = amount.replace(/\.?0+$/, "");
  if (cutAmount.includes("."))
    return cutAmount;
  return `${cutAmount}.00`;
}

export function formatPctValue(value: number | string) {
  return formatAssetAmount(value, { maxDigits: 2 }) + "%";
}
