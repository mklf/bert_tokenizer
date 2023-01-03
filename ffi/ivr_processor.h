//
// Created by lifang on 2018/11/17.
//

#ifndef BERT_TOKENIZER_BERT_TOKENIZER_H
#define BERT_TOKENIZER_BERT_TOKENIZER_H
#include <stdint.h>

#include <cstdio>
#if defined(__cplusplus)
extern "C" {
#endif

int* get_input_ids();
int* get_token_type_ids();
int* get_attention_mask();

int64_t judger_process(void* handle, const char** texts, uint32_t input_size,
                       uint32_t max_length);

const char* judger_get_error();

void* create_judger(const char*);
void drop_judger(void*);

#if defined(__cplusplus)
}
#endif

#if defined(__cplusplus)
#include <string.h>

#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
namespace tcs {

class Judger {
 private:
  void* handle;
  void copy(std::vector<int32_t>& dst, int32_t* src, int len) {
    dst.clear();
    dst.resize(len);
    memcpy(dst.data(), src, sizeof(int) * len);
  }

 public:
  Judger(const char* filename) : handle(create_judger(filename)) {
    if (handle == nullptr) {
      const char* error = judger_get_error();
      throw std::runtime_error(error);
    }
  };

  // 当输入有问题时，抛出异常
  // 返回值为-1,则继续请求模型,
  // 否则直接返回0或者1
  int64_t Process(const std::vector<std::string>& texts, uint32_t max_length,
                  std::vector<int32_t>* input_ids,
                  std::vector<int32_t>* token_type_ids,
                  std::vector<int32_t>* attention_mask) {
    const char* s[texts.size()];
    for (int i = 0; i < texts.size(); ++i) {
      s[i] = texts[i].c_str();
    }
    auto res = judger_process(handle, s, texts.size(), max_length);
    if (res == -2) {
      const char* error = judger_get_error();
      throw std::runtime_error(error);
    }
    if (res == -1) {
      copy(*input_ids, get_input_ids(), max_length);
      if (token_type_ids)
        copy(*token_type_ids, get_token_type_ids(), max_length);
      if (attention_mask)
        copy(*attention_mask, get_attention_mask(), max_length);
    }
    return res;
  }

  ~Judger() {
    if (handle) drop_judger(handle);
  }
};

}  // namespace tcs
#endif

#endif  // BERT_TOKENIZER_BERT_TOKENIZER_H
