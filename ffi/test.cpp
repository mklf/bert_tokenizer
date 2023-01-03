#include <algorithm>
#include <iostream>
#include <iterator>
#include <vector>

#include "ivr_processor.h"

int main(int argc, char* argv[]) {
  auto judger =
      tcs::Judger("x.json");

  std::vector<std::string> texts = {"进入地理槽位多轮", "大海"};
  std::vector<int32_t> input_ids, token_type_ids, attention_mask;

  int64_t ret_code =
      judger.Process(texts, 128, &input_ids, &token_type_ids, &attention_mask);
  std::cout << ret_code << "\n";

  if (ret_code != -1) {
    return 0;
  }
  std::ostream_iterator<int> oss(std::cout, ",");
  std::cout << "input_ids:";
  std::copy(input_ids.begin(), input_ids.end() - 1, oss);
  std::cout << input_ids.back();
  std::cout << "\ntoken_type_ids:";
  std::copy(token_type_ids.begin(), token_type_ids.end() - 1, oss);
  std::cout << token_type_ids.back();
  std::cout << "\nattention_mask:";
  std::copy(attention_mask.begin(), attention_mask.end() - 1, oss);
  std::cout << attention_mask.back();
  std::cout << std::endl;
  return 0;
}

// compile with :
// g++ test.cpp  -L. -ltcsivr -pthread -static -ldl -o cpp_test