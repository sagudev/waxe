initSidebarItems({"constant":[["ENCODING_NAME_MAX_LENGTH","The minimum length of buffers that may be passed to `encoding_name()`."],["INPUT_EMPTY","Return value for `*_decode_*` and `*_encode_*` functions that indicates that the input has been exhausted."],["OUTPUT_FULL","Return value for `*_decode_*` and `*_encode_*` functions that indicates that the output space has been exhausted."]],"fn":[["decoder_decode_to_utf16","Incrementally decode a byte stream into UTF-16 with malformed sequences replaced with the REPLACEMENT CHARACTER."],["decoder_decode_to_utf16_without_replacement","Incrementally decode a byte stream into UTF-16 without replacement."],["decoder_decode_to_utf8","Incrementally decode a byte stream into UTF-8 with malformed sequences replaced with the REPLACEMENT CHARACTER."],["decoder_decode_to_utf8_without_replacement","Incrementally decode a byte stream into UTF-8 without replacement."],["decoder_encoding","The `Encoding` this `Decoder` is for."],["decoder_free","Deallocates a `Decoder` previously allocated by `encoding_new_decoder()`."],["decoder_latin1_byte_compatible_up_to","Checks for compatibility with storing Unicode scalar values as unsigned bytes taking into account the state of the decoder."],["decoder_max_utf16_buffer_length","Query the worst-case UTF-16 output size (with or without replacement)."],["decoder_max_utf8_buffer_length","Query the worst-case UTF-8 output size with replacement."],["decoder_max_utf8_buffer_length_without_replacement","Query the worst-case UTF-8 output size without replacement."],["encoder_encode_from_utf16","Incrementally encode into byte stream from UTF-16 with unmappable characters replaced with HTML (decimal) numeric character references."],["encoder_encode_from_utf16_without_replacement","Incrementally encode into byte stream from UTF-16 without replacement."],["encoder_encode_from_utf8","Incrementally encode into byte stream from UTF-8 with unmappable characters replaced with HTML (decimal) numeric character references."],["encoder_encode_from_utf8_without_replacement","Incrementally encode into byte stream from UTF-8 without replacement."],["encoder_encoding","The `Encoding` this `Encoder` is for."],["encoder_free","Deallocates an `Encoder` previously allocated by `encoding_new_encoder()`."],["encoder_has_pending_state","Returns `true` if this is an ISO-2022-JP encoder that's not in the ASCII state and `false` otherwise."],["encoder_max_buffer_length_from_utf16_if_no_unmappables","Query the worst-case output size when encoding from UTF-16 with replacement."],["encoder_max_buffer_length_from_utf16_without_replacement","Query the worst-case output size when encoding from UTF-16 without replacement."],["encoder_max_buffer_length_from_utf8_if_no_unmappables","Query the worst-case output size when encoding from UTF-8 with replacement."],["encoder_max_buffer_length_from_utf8_without_replacement","Query the worst-case output size when encoding from UTF-8 without replacement."],["encoding_ascii_valid_up_to","Validates ASCII."],["encoding_can_encode_everything","Checks whether the output encoding of this encoding can encode every Unicode scalar. (Only true if the output encoding is UTF-8.)"],["encoding_for_bom","Performs non-incremental BOM sniffing."],["encoding_for_label","Implements the get an encoding algorithm."],["encoding_for_label_no_replacement","This function behaves the same as `encoding_for_label()`, except when `encoding_for_label()` would return `REPLACEMENT_ENCODING`, this method returns `NULL` instead."],["encoding_is_ascii_compatible","Checks whether the bytes 0x00...0x7F map exclusively to the characters U+0000...U+007F and vice versa."],["encoding_is_single_byte","Checks whether this encoding maps one byte to one Basic Multilingual Plane code point (i.e. byte length equals decoded UTF-16 length) and vice versa (for mappable characters)."],["encoding_iso_2022_jp_ascii_valid_up_to","Validates ISO-2022-JP ASCII-state data."],["encoding_name","Writes the name of the given `Encoding` to a caller-supplied buffer as ASCII and returns the number of bytes / ASCII characters written."],["encoding_new_decoder","Allocates a new `Decoder` for the given `Encoding` on the heap with BOM sniffing enabled and returns a pointer to the newly-allocated `Decoder`."],["encoding_new_decoder_into","Allocates a new `Decoder` for the given `Encoding` into memory provided by the caller with BOM sniffing enabled. (In practice, the target should likely be a pointer previously returned by `encoding_new_decoder()`.)"],["encoding_new_decoder_with_bom_removal","Allocates a new `Decoder` for the given `Encoding` on the heap with BOM removal and returns a pointer to the newly-allocated `Decoder`."],["encoding_new_decoder_with_bom_removal_into","Allocates a new `Decoder` for the given `Encoding` into memory provided by the caller with BOM removal."],["encoding_new_decoder_without_bom_handling","Allocates a new `Decoder` for the given `Encoding` on the heap with BOM handling disabled and returns a pointer to the newly-allocated `Decoder`."],["encoding_new_decoder_without_bom_handling_into","Allocates a new `Decoder` for the given `Encoding` into memory provided by the caller with BOM handling disabled."],["encoding_new_encoder","Allocates a new `Encoder` for the given `Encoding` on the heap and returns a pointer to the newly-allocated `Encoder`. (Exception, if the `Encoding` is `replacement`, a new `Decoder` for UTF-8 is instantiated (and that `Decoder` reports `UTF_8` as its `Encoding`)."],["encoding_new_encoder_into","Allocates a new `Encoder` for the given `Encoding` into memory provided by the caller. (In practice, the target should likely be a pointer previously returned by `encoding_new_encoder()`.)"],["encoding_output_encoding","Returns the output encoding of this encoding. This is UTF-8 for UTF-16BE, UTF-16LE and replacement and the encoding itself otherwise."],["encoding_utf8_valid_up_to","Validates UTF-8."]],"static":[["BIG5_ENCODING","The Big5 encoding."],["EUC_JP_ENCODING","The EUC-JP encoding."],["EUC_KR_ENCODING","The EUC-KR encoding."],["GB18030_ENCODING","The gb18030 encoding."],["GBK_ENCODING","The GBK encoding."],["IBM866_ENCODING","The IBM866 encoding."],["ISO_2022_JP_ENCODING","The ISO-2022-JP encoding."],["ISO_8859_10_ENCODING","The ISO-8859-10 encoding."],["ISO_8859_13_ENCODING","The ISO-8859-13 encoding."],["ISO_8859_14_ENCODING","The ISO-8859-14 encoding."],["ISO_8859_15_ENCODING","The ISO-8859-15 encoding."],["ISO_8859_16_ENCODING","The ISO-8859-16 encoding."],["ISO_8859_2_ENCODING","The ISO-8859-2 encoding."],["ISO_8859_3_ENCODING","The ISO-8859-3 encoding."],["ISO_8859_4_ENCODING","The ISO-8859-4 encoding."],["ISO_8859_5_ENCODING","The ISO-8859-5 encoding."],["ISO_8859_6_ENCODING","The ISO-8859-6 encoding."],["ISO_8859_7_ENCODING","The ISO-8859-7 encoding."],["ISO_8859_8_ENCODING","The ISO-8859-8 encoding."],["ISO_8859_8_I_ENCODING","The ISO-8859-8-I encoding."],["KOI8_R_ENCODING","The KOI8-R encoding."],["KOI8_U_ENCODING","The KOI8-U encoding."],["MACINTOSH_ENCODING","The macintosh encoding."],["REPLACEMENT_ENCODING","The replacement encoding."],["SHIFT_JIS_ENCODING","The Shift_JIS encoding."],["UTF_16BE_ENCODING","The UTF-16BE encoding."],["UTF_16LE_ENCODING","The UTF-16LE encoding."],["UTF_8_ENCODING","The UTF-8 encoding."],["WINDOWS_1250_ENCODING","The windows-1250 encoding."],["WINDOWS_1251_ENCODING","The windows-1251 encoding."],["WINDOWS_1252_ENCODING","The windows-1252 encoding."],["WINDOWS_1253_ENCODING","The windows-1253 encoding."],["WINDOWS_1254_ENCODING","The windows-1254 encoding."],["WINDOWS_1255_ENCODING","The windows-1255 encoding."],["WINDOWS_1256_ENCODING","The windows-1256 encoding."],["WINDOWS_1257_ENCODING","The windows-1257 encoding."],["WINDOWS_1258_ENCODING","The windows-1258 encoding."],["WINDOWS_874_ENCODING","The windows-874 encoding."],["X_MAC_CYRILLIC_ENCODING","The x-mac-cyrillic encoding."],["X_USER_DEFINED_ENCODING","The x-user-defined encoding."]],"struct":[["ConstEncoding","Newtype for `*const Encoding` in order to be able to implement `Sync` for it."]]});