export function parseString(value: string): boolean | string | number {
  if (value.length === 4 || value.length === 5) {
    const lowercase = value.toLowerCase();

    if (lowercase === 'true') return true;
    if (lowercase === 'false') return false;
  }
  const number = Number(value);
  if (number === 0 && !value.includes('0')) {
    return value;
  }
  if (!Number.isNaN(number)) return number;
  return value;
}
