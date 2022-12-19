#include <algorithm>
#include <iostream>
#include <iterator>
#include <vector>

#include "bert_tokenizer.h"

int main(int argc, char* argv[]) {
  tcs::ZhTokenizer& tokenizer = tcs::ZhTokenizer::Instance();

  std::vector<std::string> texts = {"进入地理槽位多轮", "今天的天气"};
  std::vector<std::vector<int>> input_ids(texts.size()),
      token_type_ids(texts.size());

  std::string text_b;
  int a[] = {228, 189, 160, 229, 165, 189, 229, 149, 138};
  for (int k = 0; k < sizeof(a) / sizeof(int); ++k) {
    text_b.push_back(a[k]);
  }
  std::cout << text_b << std::endl;

  std::ostream_iterator<int> oss(std::cout, ",");
  for (size_t i = 0; i < texts.size(); ++i) {
    tokenizer.convert_pairs_var_len(texts[i], text_b, input_ids[i],
                                    token_type_ids[i]);
    std::cout << texts[i] << "|" << text_b << "\n";
    std::cout << "input_ids:";
    std::copy(input_ids[i].begin(), input_ids[i].end(), oss);
    std::cout << "\n";
    std::cout << "token_type_ids:";
    std::copy(token_type_ids[i].begin(), token_type_ids[i].end(), oss);
    std::cout << std::endl;
  }

  return 0;
}

// compile with :
// g++ test.cpp -Lbert_tokenizer -lbert_tokenizer -pthread -static -ldl -o
// cpp_test