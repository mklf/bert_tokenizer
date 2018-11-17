# -*- coding:utf-8 -*-
import ctypes
import os.path as path
import sys

suffix = {
    "linux":".so",
    "win32":".dll",
    "darwin":".dylib"
}
if sys.platform not in suffix:
    raise NotImplementedError("platform not support, support: {}".format(suffix.keys()))

libpath = path.join(path.dirname(__file__),"libbert_tokenizer"+suffix[sys.platform])

_lib = ctypes.CDLL(libpath)

_create_full_tokenizer = _lib.create_full_tokenizer
_create_full_tokenizer.argtypes = [ctypes.c_char_p,ctypes.c_int]
_create_full_tokenizer.restype = ctypes.c_void_p

_drop_tokenizer = _lib.drop_tokenizer
_drop_tokenizer.argtypes = [ctypes.c_void_p]
_drop_tokenizer.restype = None

_convert_pairs = _lib.convert_pairs
_convert_pairs.argtypes = [ctypes.c_void_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_int]
_convert_pairs.restype = None

_get_input_ids = _lib.get_input_ids
_get_input_mask = _lib.get_input_mask
_get_segment_ids = _lib.get_segment_ids

_get_input_ids.restype = ctypes.POINTER(ctypes.c_int)
_get_input_mask.restype = ctypes.POINTER(ctypes.c_int)
_get_segment_ids.restype = ctypes.POINTER(ctypes.c_int)

_get_error = _lib.get_error
_get_error.restype = ctypes.c_char_p

class TokenizerError(Exception):
    def __init__(self,value):
        self.value = value
    def __str__(self):
        return repr(self.value)

class FullTokenizer(object):
    def __init__(self,vocab_file):
        vocab_file = ctypes.c_char_p(vocab_file.encode('utf8'))
        self.handle = _create_full_tokenizer(vocab_file,1)
        if self.handle is None:
            error_msg = _get_error().decode('utf8')
            raise TokenizerError(error_msg)


    def convert_pairs(self,text_a,text_b,max_seq_len):
        text_a = ctypes.c_char_p(text_a.encode('utf8'))
        text_b = ctypes.c_char_p(text_b.encode('utf8'))
        _convert_pairs(self.handle,text_a,text_b,ctypes.c_int(max_seq_len))
        input_ids = _get_input_ids()[:max_seq_len]
        input_mask = _get_input_mask()[:max_seq_len]
        segmet_ids = _get_segment_ids()[:max_seq_len]
        return input_ids,input_mask,segmet_ids


    def __del__(self):
        _drop_tokenizer(self.handle)
