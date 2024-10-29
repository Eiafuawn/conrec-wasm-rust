import type { TextData } from 'cheminfo-types';
import isutf8 from 'isutf8';

export interface EnsureStringOptions {
  /**
   * Specify the encoding, by default 'utf8' or 'utf16'
   * @default 'utf8' or utf16 if there is BOM utf16 or latin1 if it is not utf8
   */
  encoding?: string;
}

/**
 * Ensure that the data is string. If it is an ArrayBuffer it will be converted to string using TextDecoder.
 * @param blob
 * @param options
 * @returns
 */
export function ensureString(
  blob: TextData,
  options: EnsureStringOptions = {},
): string {
  if (typeof blob === 'string') {
    return blob;
  }
  if (ArrayBuffer.isView(blob) || blob instanceof ArrayBuffer) {
    const { encoding = guessEncoding(blob) } = options;
    const decoder = new TextDecoder(encoding);
    return decoder.decode(blob);
  }
  throw new TypeError(`blob must be a string, ArrayBuffer or ArrayBufferView`);
}

function guessEncoding(blob: ArrayBuffer | Uint8Array): string {
  const uint8 = ArrayBuffer.isView(blob)
    ? new Uint8Array(blob.buffer, blob.byteOffset, blob.byteLength)
    : new Uint8Array(blob);
  if (uint8.length >= 2) {
    if (uint8[0] === 0xfe && uint8[1] === 0xff) {
      return 'utf-16be';
    }
    if (uint8[0] === 0xff && uint8[1] === 0xfe) {
      return 'utf-16le';
    }
  }
  //@ts-expect-error an ArrayBuffer is also ok
  if (!isutf8(blob)) return 'latin1';

  return 'utf-8';
}
