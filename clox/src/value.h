#ifndef clox_value_h
#define clox_value_h

#include "common.h"

typedef double Value;

typedef struct {
  int capacity;
  int count;
  Value* values;
} ValueArray;

void initValueArray(ValueArray* chunk);
void freeValueArray(ValueArray* chunk);
void writeValueArray(ValueArray* chunk, Value value);
void printValue(Value value);

#endif
