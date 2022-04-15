//
// Created by lifang on 2018/11/17.
//

#ifndef BERT_TOKENIZER_BERT_TOKENIZER_H
#define BERT_TOKENIZER_BERT_TOKENIZER_H

#include <cstdint>

#if defined(__cplusplus)
extern "C" {
#endif

void *create_full_tokenizer(const char *vocab_file, int do_lower_case);

void drop_tokenizer(void *handle);

int convert_pairs(void *handle, const char *text_a, const char *text_b,
                  int max_seq_len, int is_pair);

int64_t *get_input_ids();
int64_t *get_input_mask();
int64_t *get_segment_ids();

const char *get_error();

#if defined(__cplusplus)
}
#endif

#if defined(__cplusplus)
#include <string.h>

#include <stdexcept>
#include <string>
#include <vector>

class FullTokenizer {
 private:
  void *handle;

 private:
  void copy(std::vector<int64_t> &dst, int64_t *src, int len) {
    dst.clear();
    dst.resize(len);
    memcpy(dst.data(), src, sizeof(int) * len);
  }

 public:
  FullTokenizer(const std::string &vocab_file, int do_lower_case)
      : handle(create_full_tokenizer(vocab_file.c_str(), do_lower_case)) {
    if (handle == nullptr) {
      const char *error = get_error();
      throw std::runtime_error(error);
    }
  };

  ~FullTokenizer() {
    if (handle) drop_tokenizer(handle);
  }

  void convert_pairs(const std::string &text_a, const std::string &text_b,
                     int max_seq_len, int is_pair,
                     std::vector<int64_t> &input_ids,
                     std::vector<int64_t> &input_mask,
                     std::vector<int64_t> &segment_ids) {
    auto seq_len = ::convert_pairs(handle, text_a.c_str(), text_b.c_str(),
                                   max_seq_len, is_pair);
    copy(input_ids, get_input_ids(), seq_len);
    copy(input_mask, get_input_mask(), seq_len);
    copy(segment_ids, get_segment_ids(), seq_len);
  }
};
#endif

#endif  // BERT_TOKENIZER_BERT_TOKENIZER_H
