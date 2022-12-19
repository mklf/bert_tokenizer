//
// Created by lifang on 2018/11/17.
//

#ifndef BERT_TOKENIZER_BERT_TOKENIZER_H
#define BERT_TOKENIZER_BERT_TOKENIZER_H
#include <cstdio>
#if defined(__cplusplus)
extern "C" {
#endif

void* create_full_tokenizer(int do_lower_case);

void drop_tokenizer(void* handle);

int convert_pairs(void* handle, const char* text_a, const char* text_b,
                  int max_seq_len, int is_pair = 1);
int* get_input_ids();
int* get_input_mask();
int* get_segment_ids();

const char* bert_tokenizer_get_error();

#if defined(__cplusplus)
}
#endif

#if defined(__cplusplus)
#include <string.h>

#include <stdexcept>
#include <string>
#include <vector>
namespace tcs {
class ZhTokenizer {
 private:
  void* handle;

 private:
  void copy(std::vector<int>& dst, int* src, int len) {
    dst.clear();
    dst.resize(len);
    memcpy(dst.data(), src, sizeof(int) * len);
  }

 public:
  ZhTokenizer() : handle(create_full_tokenizer(true)) {
    if (handle == nullptr) {
      const char* error = bert_tokenizer_get_error();
      throw std::runtime_error(error);
    }
  };

  static ZhTokenizer& Instance() {
    static ZhTokenizer tokenizer;
    return tokenizer;
  }

  ~ZhTokenizer() {
    printf("drop\n");
    if (handle) drop_tokenizer(handle);
  }

  void convert_pairs_var_len(const std::string& text_a,
                             const std::string& text_b,
                             std::vector<int>& input_ids,
                             std::vector<int>& segment_ids) {
    int max_seq_len =
        ::convert_pairs(handle, text_a.c_str(), text_b.c_str(), 0);
    if (max_seq_len == -1) throw std::runtime_error(bert_tokenizer_get_error());
    copy(input_ids, get_input_ids(), max_seq_len);
    copy(segment_ids, get_segment_ids(), max_seq_len);
  }
};
}  // namespace tcs
#endif

#endif  // BERT_TOKENIZER_BERT_TOKENIZER_H
