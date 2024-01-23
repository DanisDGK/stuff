#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void sort_string(char *str) {
  int len = strlen(str);
  for (int i = 0; i < len - 1; i++) {
    for (int j = i + 1; j < len; j++) {
      if (str[i] > str[j]) {
        char temp = str[i];
        str[i] = str[j];
        str[j] = temp;
      }
    }
  }
}

int main() {
  char *input = NULL;
  size_t len = 0;
  ssize_t nread;

  printf("Enter a sentence (CTRL+D to exit): ");
  while ((nread = getline(&input, &len, stdin)) != -1) {
    // Remove newline character, if present
    if (input[nread - 1] == '\n') {
      input[nread - 1] = '\0';
      nread--;
    }

    char *word = strtok(input, " ");
    while (word != NULL) {
      sort_string(word);
      printf("%s ", word);
      word = strtok(NULL, " ");
    }
    printf("\n");

    printf("Enter a sentence (CTRL+D to exit): ");
  }

  free(input);
  return 0;
}
