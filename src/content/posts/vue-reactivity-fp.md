---
title: 'Vue 반응성 유지하는 커링과 파이프 직접 구현하기'
description: 'Vue의 ref와 computed를 지원하는 reactiveCurry, pipe 함수를 TypeScript로 구현하는 방법'
date: '2025-08-17'
category: 'Vue'
tags: ['Vue', 'FP', 'TypeScript', 'Reactivity']
readTime: '8분 읽기'
---

# Vue 반응성을 유지하기 위한 FP 함수

Vue3에서 함수형 프로그래밍을 하려면 반응성 문제를 해결해야 한다. 일반적인 FP 라이브러리들은 Vue의 `ref`를 모르기 때문에 `.value`를 수동으로 처리해야 하고, 반응성도 깨진다.

그래서 Vue의 반응성을 알아서 처리해주는 커링과 파이프 함수를 직접 구현했다.

## 타입 구현 - 단계별로 차근차근

타입을 한번에 다 만들면 머리가 아프니까, 작은 것부터 차근차근 만들어보자.

### 1단계: MaybeRef 타입

첫 번째로 해결해야 할 문제는 함수 인자가 일반 값일 수도 있고 `Ref<T>`일 수도 있다는 것이다.

```typescript
import { Ref } from 'vue';

type MaybeRef<T> = T | Ref<T>;
```

이제 `MaybeRef<number>`라고 하면 `number | Ref<number>`가 된다.

### 2단계: Drop 타입 (배열에서 앞의 N개 제거)

부분 적용을 하려면 "앞의 2개 인자는 이미 받았으니까 나머지만 받자" 이런 걸 타입으로 표현해야 한다.

```typescript
// 기본 아이디어: [A, B, C, D]에서 앞의 2개를 빼면 [C, D]
type Drop<T extends unknown[], N extends number, I extends unknown[] = []> = 
  I['length'] extends N  // 카운터 I가 N과 같으면
    ? T                  // 그대로 반환
    : T extends [unknown, ...infer Rest]  // 첫 번째 요소 제거
      ? Drop<Rest, N, [unknown, ...I]>   // 재귀 + 카운터 증가
      : T;
```

예시로 보면:
- `Drop<[A, B, C], 2>` → `[C]`
- `Drop<[A, B, C, D], 1>` → `[B, C, D]`

### 3단계: 간단한 커링 타입부터

복잡한 커링 말고 일단 "2개 인자 받는 함수"만 커링해보자.

```typescript
// 2개 인자만 받는 단순한 버전
type SimpleCurry<A, B, R> = {
  (a: MaybeRef<A>): (b: MaybeRef<B>) => ComputedRef<R>;
  (a: MaybeRef<A>, b: MaybeRef<B>): ComputedRef<R>;
}
```

이게 기본 아이디어다. 인자를 하나씩 받거나 한번에 다 받거나.

### 4단계: 일반화된 커링 타입

이제 N개 인자로 확장해보자. 여기서부터 좀 복잡해진다.

```typescript
type CurriedFn<T extends unknown[], R> = <
  U extends unknown[] & { [K in keyof U]: K extends keyof T ? MaybeRef<T[K]> : never }
>(
  ...args: U
) => U['length'] extends T['length'] 
  ? ComputedRef<R>  // 모든 인자를 받았으면 최종 결과
  : CurriedFn<Drop<T, U['length'], []>, R>;  // 아니면 계속 커링
```

차근차근 뜯어보면:
1. `U extends unknown[]`: 받을 인자들의 타입 배열
2. `{ [K in keyof U]: ... }`: 각 인자를 `MaybeRef`로 변환
3. `U['length'] extends T['length']`: 모든 인자를 받았는지 체크
4. `Drop<T, U['length'], []>`: 받은 만큼 제거해서 남은 인자 타입들

### 5단계: 실제 사용해보기

타입이 제대로 작동하는지 확인해보자.

```typescript
// 3개 인자 받는 함수
type AddThree = (a: number, b: number, c: number) => number;

// 커링 적용
type CurriedAddThree = CurriedFn<[number, number, number], number>;

// 사용 예시
const curriedAdd: CurriedAddThree = /* 구현 */;

const step1 = curriedAdd(1);        // CurriedFn<[number, number], number>
const step2 = step1(2);             // CurriedFn<[number], number>  
const result = step2(3);            // ComputedRef<number>
```

이렇게 단계별로 타입이 줄어드는것을 확인할 수 있다.

## reactiveCurry 구현

이제 실제 구현을 보자. 핵심은 `isRef`로 ref인지 체크하고 `computed` 안에서 `.value`를 처리하는 것이다.

```typescript
export function reactiveCurry<T extends unknown[], R>(fn: (...args: T) => R): CurriedFn<T, R> {
  function curried<U extends unknown[]>(
    ...args: U
  ): U['length'] extends T['length'] ? ComputedRef<R> : CurriedFn<Drop<T, U['length'], []>, R> {
    type Result = U['length'] extends T['length'] ? ComputedRef<R> : CurriedFn<Drop<T, U['length'], []>, R>;
    
    if (args.length >= fn.length) {
      // 모든 인자를 받았으면 computed로 감싸서 반환
      return computed(() => fn(...(args.map((arg) => (isRef(arg) ? arg.value : arg)) as T))) as unknown as Result;
    } else {
      // 아직 인자가 부족하면 계속 커링
      return ((...more: unknown[]) => curried(...args, ...more)) as unknown as Result;
    }
  }
  return curried as CurriedFn<T, R>;
}
```

핵심 포인트:
1. `args.length >= fn.length`로 모든 인자를 받았는지 체크
2. 받았으면 `computed(() => ...)` 안에서 실제 함수 실행
3. `isRef(arg) ? arg.value : arg`로 ref인 경우만 `.value` 접근
4. 아직 인자가 부족하면 새로운 커링 함수 반환

## curryRight 구현: 인자 순서 뒤집기

오른쪽부터 인자를 채우려면 함수의 인자 순서를 뒤집어야 한다.

```typescript
export function reactiveCurryRight<T extends unknown[], R>(fn: (...args: T) => R): CurriedFn<Reverse<T>, R> {
  const reversedFn = (...args: unknown[]): R => {
    const reversedArgs = args.slice().reverse();
    return fn(...(reversedArgs as T));
  };

  Object.defineProperty(reversedFn, 'length', { value: fn.length });
  return reactiveCurry(reversedFn) as unknown as CurriedFn<Reverse<T>, R>;
}
```

### Reverse 타입 구현

타입 레벨에서도 배열을 뒤집어야 한다.

```typescript
export type Reverse<T extends unknown[]> = T extends []
  ? []
  : T extends [infer F, ...infer Rest]
    ? [...Reverse<Rest>, F]
    : T;
```

재귀적으로 첫 번째 요소를 빼고 나머지를 뒤집은 다음 맨 끝에 붙인다.

## 실제 사용 예시

```typescript
const add = (a: number, b: number, c: number) => a + b + c;
const reactiveAdd = reactiveCurry(add);

const num1 = ref(1);
const num2 = ref(2);

// 부분 적용
const addWithFirst = reactiveAdd(num1); // CurriedFn<[number, number], number>
const addWithTwo = addWithFirst(num2);  // CurriedFn<[number], number>
const result = addWithTwo(ref(3));      // ComputedRef<number>
```

타입 추론이 각 단계에서 정확하게 작동한다.

## 비반응형 버전

반응성이 필요 없는 경우를 위한 일반 커링도 구현했다.

```typescript
export function curry<T extends unknown[], R>(fn: (...args: T) => R): NonReactiveCurriedFn<T, R> {
  function curried<U extends unknown[]>(
    ...args: U
  ): U['length'] extends T['length'] ? R : NonReactiveCurriedFn<Drop<T, U['length'], []>, R> {
    type Result = U['length'] extends T['length'] ? R : NonReactiveCurriedFn<Drop<T, U['length'], []>, R>;
    if (args.length >= fn.length) {
      // computed 없이 바로 실행
      return fn(...(args.map((arg) => (isRef(arg) ? (arg as { value: unknown }).value : arg)) as T)) as Result;
    } else {
      return ((...more: unknown[]) => curried(...args, ...more)) as Result;
    }
  }
  return curried as NonReactiveCurriedFn<T, R>;
}
```

차이점은 `computed`로 감싸지 않고 바로 실행한다는 것이다.

## pipe 함수 구현

여러 함수를 체이닝하는 pipe는 TypeScript 한계 때문에 오버로드로 구현했다.

```typescript
// 최대 10개까지 오버로드 정의
export function pipe<A extends any[], R>(fn: (...args: A) => R): (...args: A) => R;
export function pipe<A extends any[], T1, R>(f1: (...args: A) => T1, f2: (a: T1) => R): (...args: A) => R;
// ... 계속

export function pipe(...fns: Fn[]): Fn {
  return (...args: any[]) => {
    const [first, ...rest] = fns;
    const initial = first(...args);
    return rest.reduce((prev, fn) => fn(prev), initial);
  };
}
```

실제 구현은 단순하지만 타입 안전성을 위해 오버로드가 필요하다.

## 타입 구현 팁

1. **작은 유틸리티 타입부터**: `Drop`, `Reverse` 같은 기본 타입을 먼저 구현
2. **재귀 타입 활용**: TypeScript의 재귀 타입을 적극 활용
3. **조건부 타입**: `extends`를 써서 타입 레벨 로직 구현
4. **템플릿 리터럴**: 필요하면 문자열 타입도 조작 가능
5. **오버로드**: 복잡한 경우 런타임 구현과 타입을 분리

## 실제 프로젝트에서 사용

```typescript
// 복잡한 계산 로직
const calculatePrice = (basePrice: number, discount: number, tax: number) => 
  (basePrice - discount) * (1 + tax);

const reactiveCalculate = reactiveCurry(calculatePrice);

const basePrice = ref(100);
const discount = ref(10);
const taxRate = ref(0.1);

// 부분 적용으로 재사용 가능한 함수 생성
const withBase = reactiveCalculate(basePrice);
const withDiscount = withBase(discount);
const finalPrice = withDiscount(taxRate); // ComputedRef<number>

// basePrice나 discount가 바뀌면 자동으로 재계산됨
```

이런 식으로 복잡한 비즈니스 로직도 함수형으로 깔끔하게 처리할 수 있다.

## 마무리

Vue의 반응성과 함수형 프로그래밍을 결합하려면:

1. **타입 설계가 핵심**: `MaybeRef`부터 시작해서 복잡한 커링 타입까지
2. **런타임에서 `isRef` 체크**: ref인지 일반 값인지 구분해서 처리
3. **`computed`로 반응성 유지**: 최종 결과를 computed로 감싸기
4. **TypeScript 타입 시스템 활용**: 재귀, 조건부 타입으로 복잡한 로직 표현

한번 구현해놓으면 정말 편하다. Vue 프로젝트에서 함수형의 장점을 그대로 살릴 수 있다.