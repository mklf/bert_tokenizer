//
// Created by lifang on 2018/11/17.
//

#ifndef BERT_TOKENIZER_BERT_TOKENIZER_H
#define BERT_TOKENIZER_BERT_TOKENIZER_H
#if defined(__cplusplus)
extern "C" {
#endif

void* create_full_tokenizer(const char* vocab_file, int do_lower_case);

void drop_tokenizer(void* handle);

int convert_pairs(void* handle, const char* text_a, const char* text_b, int max_seq_len);
int* get_input_ids();
int* get_input_mask();
int* get_segment_ids();

const char* bert_tokenizer_get_error();

#if defined(__cplusplus)
}
#endif

#if defined(__cplusplus)
#  include <string.h>
#  include <stdexcept>
#  include <string>
#  include <vector>

class FullTokenizer {
 private:
  void* handle;

 private:
  void copy(std::vector<int>& dst, int* src, int len) {
    dst.clear();
    dst.resize(len);
    memcpy(dst.data(), src, sizeof(int) * len);
  }

 public:
  FullTokenizer(const std::string& vocab_file, int do_lower_case)
      : handle(create_full_tokenizer(vocab_file.c_str(), do_lower_case)) {
    if (handle == nullptr) {
      const char* error = bert_tokenizer_get_error();
      throw std::runtime_error(error);
    }
  };

  ~FullTokenizer() {
    // if(handle) drop_tokenizer(handle);
  }

  void convert_pairs(const std::string& text_a, const std::string& text_b, int max_seq_len, std::vector<int>& input_ids,
                     std::vector<int>& input_mask, std::vector<int>& segment_ids) {
    ::convert_pairs(handle, text_a.c_str(), text_b.c_str(), max_seq_len);
    copy(input_ids, get_input_ids(), max_seq_len);
    copy(input_mask, get_input_mask(), max_seq_len);
    copy(segment_ids, get_segment_ids(), max_seq_len);
  }

  void convert_pairs_var_len(const std::string& text_a, const std::string& text_b, std::vector<int>& input_ids,
                             std::vector<int>& segment_ids) {
    int max_seq_len = ::convert_pairs(handle, text_a.c_str(), text_b.c_str(), 0);
    copy(input_ids, get_input_ids(), max_seq_len);
    copy(segment_ids, get_segment_ids(), max_seq_len);
  }
};
#endif

#endif  // BERT_TOKENIZER_BERT_TOKENIZER_H
