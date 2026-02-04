---
title: '나만의 렌더링 엔진 만들기: 가상 DOM부터 시그널까지'
description: '가상 DOM과 시그널 기반 반응성 시스템을 직접 구현하며 렌더링의 핵심 원리를 이해합니다.'
date: '2026-02-04'
category: 'Render'
tags: ['DOM', 'Render', 'Virtual DOM', 'Reactivity']
readTime: '40분 읽기'
---

# 나만의 렌더링 엔진 만들기

[이전 글](/posts/render-1)에서 웹 렌더링의 진화 과정을 살펴보았다. 바닐라 JavaScript부터 React의 Fiber 아키텍처, 그리고 Solid.js의 시그널까지 다양한 렌더링 방식을 이론적으로 분석했다.

이론을 완전히 이해하는 가장 좋은 방법은 직접 만들어보는 것이다. 이번 글에서는 두 가지 렌더링 엔진을 처음부터 구현해본다:

1. **가상 DOM 기반 렌더링 엔진** - React 스타일
2. **시그널 기반 반응성 시스템** - Solid.js 스타일

같은 Todo 앱을 두 가지 방식으로 구현하면서 각 접근법의 차이점을 체감해보자.

---

## Part 1: 가상 DOM 기반 렌더링 엔진

### 가상 DOM이란?

가상 DOM은 실제 DOM의 경량 복사본이다. JavaScript 객체로 UI 구조를 표현하고, 변경이 발생하면 이전 가상 DOM과 새 가상 DOM을 비교(diff)하여 실제로 변경된 부분만 DOM에 반영(patch)한다.

```
상태 변경 → 새 가상 DOM 생성 → diff → patch → 실제 DOM 업데이트
```

이 과정을 단계별로 구현해보자.

### Step 1: 가상 노드(VNode) 생성

먼저 가상 DOM 노드를 생성하는 `createElement` 함수를 만든다.

```javascript
/**
 * 가상 DOM 노드를 생성한다.
 * @param {string} type - 태그명 (예: 'div', 'span')
 * @param {object} props - 속성 객체
 * @param {...any} children - 자식 노드들
 * @returns {object} VNode 객체
 */
function createElement(type, props = {}, ...children) {
  return {
    type,
    props: props || {},
    children: children
      .flat() // 중첩 배열 평탄화
      .map(child =>
        // 문자열/숫자는 텍스트 노드로 변환
        typeof child === 'object' ? child : createTextNode(child)
      ),
  };
}

/**
 * 텍스트 노드를 생성한다.
 */
function createTextNode(text) {
  return {
    type: 'TEXT',
    props: { nodeValue: String(text) },
    children: [],
  };
}

// 사용 예시
const vdom = createElement('div', { id: 'app' },
  createElement('h1', { className: 'title' }, 'Hello'),
  createElement('p', null, 'World')
);

// 결과:
// {
//   type: 'div',
//   props: { id: 'app' },
//   children: [
//     { type: 'h1', props: { className: 'title' }, children: [{ type: 'TEXT', ... }] },
//     { type: 'p', props: {}, children: [{ type: 'TEXT', ... }] }
//   ]
// }
```

이 함수는 JSX가 변환되는 형태와 동일하다. Babel은 JSX를 다음과 같이 변환한다:

```jsx
// JSX
<div id="app">
  <h1 className="title">Hello</h1>
</div>

// 변환 후
createElement('div', { id: 'app' },
  createElement('h1', { className: 'title' }, 'Hello')
)
```

### Step 2: 가상 DOM을 실제 DOM으로 렌더링

VNode를 실제 DOM 요소로 변환하는 `render` 함수를 구현한다.

```javascript
/**
 * VNode를 실제 DOM 요소로 변환한다.
 * @param {object} vnode - 가상 DOM 노드
 * @returns {HTMLElement|Text} 실제 DOM 노드
 */
function render(vnode) {
  // 텍스트 노드 처리
  if (vnode.type === 'TEXT') {
    return document.createTextNode(vnode.props.nodeValue);
  }

  // 요소 노드 생성
  const element = document.createElement(vnode.type);

  // props 적용
  applyProps(element, {}, vnode.props);

  // 자식 노드 재귀적으로 렌더링
  vnode.children
    .map(render)
    .forEach(child => element.appendChild(child));

  return element;
}

/**
 * DOM 요소에 props를 적용한다.
 * @param {HTMLElement} element - 대상 요소
 * @param {object} oldProps - 이전 props
 * @param {object} newProps - 새 props
 */
function applyProps(element, oldProps, newProps) {
  // 제거된 props 처리
  Object.keys(oldProps).forEach(name => {
    if (!(name in newProps)) {
      if (name.startsWith('on')) {
        // 이벤트 리스너 제거
        const eventType = name.slice(2).toLowerCase();
        element.removeEventListener(eventType, oldProps[name]);
      } else if (name !== 'children') {
        // 속성 제거
        element.removeAttribute(name);
      }
    }
  });

  // 새로운/변경된 props 적용
  Object.keys(newProps).forEach(name => {
    if (name === 'children') return;

    if (oldProps[name] !== newProps[name]) {
      if (name.startsWith('on')) {
        // 이벤트 리스너 처리
        const eventType = name.slice(2).toLowerCase();
        if (oldProps[name]) {
          element.removeEventListener(eventType, oldProps[name]);
        }
        element.addEventListener(eventType, newProps[name]);
      } else if (name === 'className') {
        // className → class 변환
        element.setAttribute('class', newProps[name]);
      } else if (name === 'style' && typeof newProps[name] === 'object') {
        // 스타일 객체 처리
        Object.assign(element.style, newProps[name]);
      } else {
        // 일반 속성
        element.setAttribute(name, newProps[name]);
      }
    }
  });
}

/**
 * VNode를 컨테이너에 마운트한다.
 * @param {object} vnode - 가상 DOM 노드
 * @param {HTMLElement} container - 컨테이너 요소
 */
function mount(vnode, container) {
  const dom = render(vnode);
  container.appendChild(dom);
  return dom;
}

// 사용 예시
const app = createElement('div', { id: 'app', onClick: () => alert('clicked') },
  createElement('h1', null, 'Hello World')
);

mount(app, document.getElementById('root'));
```

### Step 3: Diff 알고리즘 구현

이제 핵심인 diff 알고리즘을 구현한다. 두 VNode를 비교하여 어떤 변경이 필요한지 계산한다.

```javascript
/**
 * 두 VNode를 비교하여 패치 정보를 반환한다.
 * @param {object} oldVNode - 이전 VNode
 * @param {object} newVNode - 새 VNode
 * @returns {object} 패치 정보
 */
function diff(oldVNode, newVNode) {
  // Case 1: 새 노드가 없음 → 제거
  if (newVNode === undefined || newVNode === null) {
    return { type: 'REMOVE' };
  }

  // Case 2: 이전 노드가 없음 → 생성
  if (oldVNode === undefined || oldVNode === null) {
    return { type: 'CREATE', newVNode };
  }

  // Case 3: 노드 타입이 다름 → 교체
  if (oldVNode.type !== newVNode.type) {
    return { type: 'REPLACE', newVNode };
  }

  // Case 4: 텍스트 노드인 경우
  if (newVNode.type === 'TEXT') {
    if (oldVNode.props.nodeValue !== newVNode.props.nodeValue) {
      return { type: 'TEXT', text: newVNode.props.nodeValue };
    }
    return { type: 'NONE' };
  }

  // Case 5: 같은 타입의 요소 노드 → props와 children 비교
  return {
    type: 'UPDATE',
    props: diffProps(oldVNode.props, newVNode.props),
    children: diffChildren(oldVNode.children, newVNode.children),
  };
}

/**
 * props를 비교한다.
 */
function diffProps(oldProps, newProps) {
  const patches = [];

  // 새로 추가되거나 변경된 props
  Object.keys(newProps).forEach(name => {
    if (oldProps[name] !== newProps[name]) {
      patches.push({ type: 'SET', name, value: newProps[name] });
    }
  });

  // 제거된 props
  Object.keys(oldProps).forEach(name => {
    if (!(name in newProps)) {
      patches.push({ type: 'REMOVE', name });
    }
  });

  return patches;
}

/**
 * 자식 노드들을 비교한다.
 * 기본적으로 인덱스 기반으로 비교한다.
 */
function diffChildren(oldChildren, newChildren) {
  const patches = [];
  const maxLength = Math.max(oldChildren.length, newChildren.length);

  for (let i = 0; i < maxLength; i++) {
    patches.push(diff(oldChildren[i], newChildren[i]));
  }

  return patches;
}
```

diff 알고리즘은 O(n) 시간 복잡도로 동작한다. 트리를 깊이 우선으로 순회하면서 같은 위치의 노드끼리 비교한다.

### Step 4: Patch 함수 구현

diff 결과를 실제 DOM에 적용하는 `patch` 함수를 구현한다.

```javascript
/**
 * 패치를 실제 DOM에 적용한다.
 * @param {HTMLElement} parent - 부모 요소
 * @param {object} patches - 패치 정보
 * @param {number} index - 자식 인덱스
 */
function patch(parent, patches, index = 0) {
  const element = parent.childNodes[index];

  switch (patches.type) {
    case 'NONE':
      // 변경 없음
      return;

    case 'CREATE':
      // 새 노드 추가
      parent.appendChild(render(patches.newVNode));
      return;

    case 'REMOVE':
      // 노드 제거
      if (element) {
        parent.removeChild(element);
      }
      return;

    case 'REPLACE':
      // 노드 교체
      parent.replaceChild(render(patches.newVNode), element);
      return;

    case 'TEXT':
      // 텍스트 변경
      element.textContent = patches.text;
      return;

    case 'UPDATE':
      // props 업데이트
      patchProps(element, patches.props);

      // children 업데이트 (역순으로 처리하여 인덱스 문제 방지)
      patches.children.forEach((childPatch, i) => {
        patch(element, childPatch, i);
      });
      return;
  }
}

/**
 * props 패치를 적용한다.
 */
function patchProps(element, propsPatches) {
  propsPatches.forEach(({ type, name, value }) => {
    if (type === 'SET') {
      if (name.startsWith('on')) {
        const eventType = name.slice(2).toLowerCase();
        // 기존 리스너 제거를 위해 element에 저장
        if (element.__listeners && element.__listeners[eventType]) {
          element.removeEventListener(eventType, element.__listeners[eventType]);
        }
        element.__listeners = element.__listeners || {};
        element.__listeners[eventType] = value;
        element.addEventListener(eventType, value);
      } else if (name === 'className') {
        element.setAttribute('class', value);
      } else if (name === 'style' && typeof value === 'object') {
        Object.assign(element.style, value);
      } else {
        element.setAttribute(name, value);
      }
    } else if (type === 'REMOVE') {
      if (name.startsWith('on')) {
        const eventType = name.slice(2).toLowerCase();
        if (element.__listeners && element.__listeners[eventType]) {
          element.removeEventListener(eventType, element.__listeners[eventType]);
          delete element.__listeners[eventType];
        }
      } else {
        element.removeAttribute(name);
      }
    }
  });
}
```

### Step 5: 상태 관리와 재렌더링

이제 상태 변경 시 자동으로 재렌더링하는 시스템을 구현한다.

```javascript
/**
 * 미니 프레임워크 - 상태 관리와 재렌더링
 */
function createApp(initialState, view) {
  let state = initialState;
  let oldVNode = null;
  let rootElement = null;

  /**
   * 상태를 업데이트하고 재렌더링한다.
   */
  function setState(updater) {
    // 함수형 업데이트 또는 객체 병합
    if (typeof updater === 'function') {
      state = updater(state);
    } else {
      state = { ...state, ...updater };
    }
    update();
  }

  /**
   * 화면을 업데이트한다.
   */
  function update() {
    const newVNode = view(state, setState);

    if (oldVNode === null) {
      // 최초 렌더링
      rootElement = render(newVNode);
      return rootElement;
    }

    // diff & patch
    const patches = diff(oldVNode, newVNode);
    patch(rootElement.parentNode, patches,
      Array.from(rootElement.parentNode.childNodes).indexOf(rootElement));

    oldVNode = newVNode;
  }

  /**
   * 앱을 컨테이너에 마운트한다.
   */
  function mountApp(container) {
    const newVNode = view(state, setState);
    rootElement = render(newVNode);
    container.appendChild(rootElement);
    oldVNode = newVNode;
  }

  return { mountApp, setState, getState: () => state };
}
```

### Step 6: Todo 앱 예제 (가상 DOM 버전)

지금까지 만든 가상 DOM 엔진으로 Todo 앱을 구현해보자.

```javascript
// Todo 앱 컴포넌트
function TodoApp(state, setState) {
  const { todos, inputValue } = state;

  const handleInput = (e) => {
    setState({ inputValue: e.target.value });
  };

  const addTodo = () => {
    if (!inputValue.trim()) return;
    setState(prev => ({
      todos: [...prev.todos, {
        id: Date.now(),
        text: prev.inputValue,
        completed: false
      }],
      inputValue: ''
    }));
  };

  const toggleTodo = (id) => {
    setState(prev => ({
      todos: prev.todos.map(todo =>
        todo.id === id ? { ...todo, completed: !todo.completed } : todo
      )
    }));
  };

  const deleteTodo = (id) => {
    setState(prev => ({
      todos: prev.todos.filter(todo => todo.id !== id)
    }));
  };

  return createElement('div', { className: 'todo-app' },
    createElement('h1', null, 'Todo List'),

    // 입력 영역
    createElement('div', { className: 'input-area' },
      createElement('input', {
        type: 'text',
        value: inputValue,
        onInput: handleInput,
        placeholder: 'What needs to be done?'
      }),
      createElement('button', { onClick: addTodo }, 'Add')
    ),

    // Todo 리스트
    createElement('ul', { className: 'todo-list' },
      ...todos.map(todo =>
        createElement('li', {
          key: todo.id,
          className: todo.completed ? 'completed' : ''
        },
          createElement('span', { onClick: () => toggleTodo(todo.id) }, todo.text),
          createElement('button', { onClick: () => deleteTodo(todo.id) }, 'Delete')
        )
      )
    ),

    // 카운터
    createElement('p', { className: 'counter' },
      `${todos.filter(t => !t.completed).length} items left`
    )
  );
}

// 앱 실행
const app = createApp(
  { todos: [], inputValue: '' },
  TodoApp
);

app.mountApp(document.getElementById('root'));
```

### 가상 DOM 방식 정리

지금까지 구현한 가상 DOM 엔진의 전체 흐름을 정리하면:

```
1. createElement() → VNode 객체 생성
2. render() → VNode를 실제 DOM으로 변환
3. 상태 변경 시:
   - 새로운 VNode 트리 생성
   - diff() → 이전/새 VNode 비교
   - patch() → 변경된 부분만 DOM 업데이트
```

약 200줄의 코드로 React의 핵심 원리를 구현했다. 물론 실제 React는 Fiber, Concurrent Mode, Hooks 등 훨씬 복잡한 기능을 포함하지만, 기본 원리는 동일하다.

---

## Part 2: 시그널 기반 반응성 시스템

이번에는 가상 DOM 없이 **세밀한 반응성(Fine-grained Reactivity)**을 구현해본다. Solid.js 스타일의 시그널 시스템이다.

### 시그널이란?

시그널은 **값의 변경을 추적할 수 있는 반응형 원시 타입**이다. 값을 읽을 때 자동으로 의존성이 등록되고, 값이 변경되면 의존하는 코드가 자동으로 재실행된다.

```javascript
const [count, setCount] = createSignal(0);

createEffect(() => {
  console.log(count()); // count를 읽으면 이 effect가 자동 등록됨
});

setCount(1); // count가 변경되면 effect가 자동 재실행
// 콘솔: 1
```

놀랍게도 이 시스템은 **50줄 미만의 코드**로 구현할 수 있다.

### Step 1: createSignal 구현

```javascript
// 현재 실행 중인 subscriber를 추적하는 전역 변수
let currentSubscriber = null;

/**
 * 반응형 시그널을 생성한다.
 * @param {any} initialValue - 초기값
 * @returns {[Function, Function]} [getter, setter]
 */
function createSignal(initialValue) {
  let value = initialValue;
  const subscribers = new Set();

  // Getter: 값을 읽고 현재 subscriber를 등록
  const read = () => {
    if (currentSubscriber) {
      subscribers.add(currentSubscriber);
    }
    return value;
  };

  // Setter: 값을 변경하고 모든 subscriber 실행
  const write = (newValue) => {
    // 함수형 업데이트 지원
    if (typeof newValue === 'function') {
      newValue = newValue(value);
    }

    // 값이 실제로 변경된 경우에만 업데이트
    if (value !== newValue) {
      value = newValue;
      // Set을 복사하여 순회 중 수정 문제 방지
      [...subscribers].forEach(subscriber => subscriber());
    }
  };

  return [read, write];
}
```

핵심 아이디어:
1. **getter 호출 시**: 현재 실행 중인 effect(subscriber)를 구독 목록에 추가
2. **setter 호출 시**: 값 변경 후 모든 subscriber를 재실행

### Step 2: createEffect 구현

```javascript
/**
 * 부수 효과(side effect)를 생성한다.
 * 내부에서 읽은 시그널의 변경을 자동으로 추적한다.
 * @param {Function} fn - 실행할 함수
 */
function createEffect(fn) {
  const execute = () => {
    // 중첩된 effect를 위해 이전 subscriber 저장
    const previousSubscriber = currentSubscriber;
    currentSubscriber = execute;

    try {
      fn();
    } finally {
      // subscriber 복원
      currentSubscriber = previousSubscriber;
    }
  };

  // 최초 실행 - 이때 의존성이 자동으로 추적됨
  execute();
}
```

동작 원리:
1. effect 함수 실행 전에 `currentSubscriber`를 자신으로 설정
2. 함수 내에서 signal을 읽으면 자동으로 구독됨
3. 실행 후 `currentSubscriber` 복원 (중첩 effect 지원)

> **주의**: effect 내부의 비동기 코드에서 signal을 읽으면 의존성이 추적되지 않는다. `currentSubscriber`가 이미 복원된 상태이기 때문이다.

### Step 3: createMemo 구현

`createMemo`는 파생 상태(computed/derived state)를 위한 함수다. signal과 effect의 조합에 **캐싱**을 추가한 것이다.

```javascript
/**
 * 메모이즈된 파생 값을 생성한다.
 * 의존성이 변경될 때만 재계산된다.
 * @param {Function} fn - 값을 계산하는 함수
 * @returns {Function} getter 함수
 */
function createMemo(fn) {
  const [value, setValue] = createSignal();
  let initialized = false;

  createEffect(() => {
    const newValue = fn();

    // 최초 실행이거나 값이 변경된 경우에만 업데이트
    // 동등성 비교로 불필요한 downstream 업데이트 방지
    if (!initialized || value() !== newValue) {
      setValue(newValue);
      initialized = true;
    }
  });

  return value;
}

// 사용 예시
const [count, setCount] = createSignal(0);
const doubled = createMemo(() => count() * 2);

createEffect(() => {
  console.log('doubled:', doubled());
});

setCount(5);  // doubled: 10
setCount(5);  // 값이 같으므로 effect 재실행 안 됨
```

`createMemo`의 핵심:
- **캐싱**: 계산 결과를 저장하여 여러 번 읽어도 재계산 없음
- **동등성 비교**: 결과가 같으면 하위 effect 트리거 안 함

### Step 4: DOM 바인딩

시그널을 DOM에 연결하는 방법을 구현한다. 가상 DOM 없이 직접 DOM을 업데이트한다.

```javascript
/**
 * 텍스트 노드를 시그널에 바인딩한다.
 * 시그널이 변경되면 텍스트도 자동 업데이트된다.
 */
function bindText(signal) {
  const textNode = document.createTextNode('');

  createEffect(() => {
    textNode.textContent = signal();
  });

  return textNode;
}

/**
 * 요소의 속성을 시그널에 바인딩한다.
 */
function bindAttribute(element, attrName, signal) {
  createEffect(() => {
    const value = signal();
    if (attrName === 'className') {
      element.className = value;
    } else if (attrName === 'style' && typeof value === 'object') {
      Object.assign(element.style, value);
    } else {
      element.setAttribute(attrName, value);
    }
  });
}

/**
 * 조건부 렌더링을 위한 헬퍼
 */
function bindShow(element, signal) {
  createEffect(() => {
    element.style.display = signal() ? '' : 'none';
  });
}
```

### Step 5: 리스트 렌더링

동적 리스트를 효율적으로 렌더링하는 `For` 함수를 구현한다.

```javascript
/**
 * 리스트를 반응형으로 렌더링한다.
 * @param {Function} listSignal - 배열을 반환하는 시그널
 * @param {Function} renderItem - 아이템을 DOM으로 변환하는 함수
 * @param {HTMLElement} container - 컨테이너 요소
 */
function For(listSignal, renderItem, container) {
  let currentItems = [];
  let currentElements = [];

  createEffect(() => {
    const newItems = listSignal();

    // 간단한 구현: 전체 교체
    // (실제로는 key 기반 최적화가 필요)
    const newElements = newItems.map((item, index) => renderItem(item, index));

    // 기존 요소 제거
    currentElements.forEach(el => el.remove());

    // 새 요소 추가
    newElements.forEach(el => container.appendChild(el));

    currentItems = newItems;
    currentElements = newElements;
  });
}

/**
 * key 기반 최적화된 리스트 렌더링
 */
function ForKeyed(listSignal, getKey, renderItem, container) {
  const elementMap = new Map(); // key → element 매핑

  createEffect(() => {
    const newItems = listSignal();
    const newKeys = new Set(newItems.map(getKey));

    // 삭제된 아이템 제거
    for (const [key, element] of elementMap) {
      if (!newKeys.has(key)) {
        element.remove();
        elementMap.delete(key);
      }
    }

    // 새 아이템 추가 또는 순서 조정
    let prevElement = null;
    newItems.forEach((item, index) => {
      const key = getKey(item);
      let element = elementMap.get(key);

      if (!element) {
        // 새 아이템
        element = renderItem(item, index);
        elementMap.set(key, element);
      }

      // 올바른 위치에 삽입
      if (prevElement) {
        if (prevElement.nextSibling !== element) {
          prevElement.after(element);
        }
      } else {
        if (container.firstChild !== element) {
          container.prepend(element);
        }
      }

      prevElement = element;
    });
  });
}
```

### Step 6: Todo 앱 예제 (시그널 버전)

같은 Todo 앱을 시그널 기반으로 구현해보자.

```javascript
function createTodoApp(container) {
  // 상태 정의
  const [todos, setTodos] = createSignal([]);
  const [inputValue, setInputValue] = createSignal('');

  // 파생 상태
  const remainingCount = createMemo(() =>
    todos().filter(todo => !todo.completed).length
  );

  // 액션
  const addTodo = () => {
    const text = inputValue().trim();
    if (!text) return;

    setTodos(prev => [...prev, {
      id: Date.now(),
      text,
      completed: false
    }]);
    setInputValue('');
  };

  const toggleTodo = (id) => {
    setTodos(prev => prev.map(todo =>
      todo.id === id ? { ...todo, completed: !todo.completed } : todo
    ));
  };

  const deleteTodo = (id) => {
    setTodos(prev => prev.filter(todo => todo.id !== id));
  };

  // UI 구성
  const app = document.createElement('div');
  app.className = 'todo-app';

  // 제목
  const title = document.createElement('h1');
  title.textContent = 'Todo List (Signal)';
  app.appendChild(title);

  // 입력 영역
  const inputArea = document.createElement('div');
  inputArea.className = 'input-area';

  const input = document.createElement('input');
  input.type = 'text';
  input.placeholder = 'What needs to be done?';
  input.addEventListener('input', (e) => setInputValue(e.target.value));

  // input value 바인딩
  createEffect(() => {
    input.value = inputValue();
  });

  const addButton = document.createElement('button');
  addButton.textContent = 'Add';
  addButton.addEventListener('click', addTodo);

  inputArea.appendChild(input);
  inputArea.appendChild(addButton);
  app.appendChild(inputArea);

  // Todo 리스트
  const todoList = document.createElement('ul');
  todoList.className = 'todo-list';

  // 리스트 렌더링
  ForKeyed(
    todos,
    todo => todo.id,
    (todo) => {
      const li = document.createElement('li');

      // completed 상태에 따른 클래스 바인딩
      // 개별 아이템의 상태를 위한 시그널 생성
      const [isCompleted, setIsCompleted] = createSignal(todo.completed);

      createEffect(() => {
        li.className = isCompleted() ? 'completed' : '';
      });

      const span = document.createElement('span');
      span.textContent = todo.text;
      span.addEventListener('click', () => {
        toggleTodo(todo.id);
        setIsCompleted(!isCompleted());
      });

      const deleteBtn = document.createElement('button');
      deleteBtn.textContent = 'Delete';
      deleteBtn.addEventListener('click', () => deleteTodo(todo.id));

      li.appendChild(span);
      li.appendChild(deleteBtn);
      return li;
    },
    todoList
  );

  app.appendChild(todoList);

  // 카운터 - 자동으로 업데이트됨
  const counter = document.createElement('p');
  counter.className = 'counter';

  createEffect(() => {
    counter.textContent = `${remainingCount()} items left`;
  });

  app.appendChild(counter);

  container.appendChild(app);
}

// 앱 실행
createTodoApp(document.getElementById('root'));
```

### 시그널 방식의 특징

시그널 기반 시스템은 가상 DOM과 완전히 다른 방식으로 동작한다:

```
시그널 변경 → 해당 시그널을 구독하는 effect만 재실행 → 직접 DOM 업데이트
```

가상 DOM처럼 전체 트리를 비교하지 않고, **변경된 값에 의존하는 코드만 정확히 재실행**한다. 이것이 "Fine-grained Reactivity"의 의미다.

---

## 두 방식 비교

### 렌더링 방식

**가상 DOM**
- **업데이트 범위**: 컴포넌트 전체 재렌더링 후 diff
- **비교 방식**: 이전/새 VNode 트리 비교
- **DOM 조작**: patch로 일괄 처리

**시그널**
- **업데이트 범위**: 변경된 부분만 직접 업데이트
- **비교 방식**: 비교 없음, 구독 기반
- **DOM 조작**: effect에서 직접 처리

### 성능 특성

```javascript
// 가상 DOM: count가 변경되면 전체 함수 재실행
function Counter({ count }) {
  return createElement('div', null,
    createElement('span', null, count),
    createElement('span', null, '고정 텍스트') // 이것도 다시 생성됨
  );
}

// 시그널: count가 변경되면 해당 텍스트 노드만 업데이트
function Counter() {
  const [count, setCount] = createSignal(0);

  const span1 = document.createElement('span');
  createEffect(() => {
    span1.textContent = count(); // 이 effect만 재실행
  });

  const span2 = document.createElement('span');
  span2.textContent = '고정 텍스트'; // 한 번만 설정됨
}
```

### 사용 사례

**가상 DOM이 유리한 경우**
- 복잡한 UI 상태 관리
- 팀 협업 (예측 가능성)
- 서버 사이드 렌더링
- 풍부한 생태계 필요

**시그널이 유리한 경우**
- 실시간 업데이트가 많은 경우
- 성능이 중요한 경우
- 번들 크기 최소화
- 세밀한 제어 필요

---

## 마무리

이 글에서 두 가지 렌더링 엔진을 직접 구현해보았다:

1. **가상 DOM 방식**: createElement → render → diff → patch
2. **시그널 방식**: createSignal → createEffect → createMemo

두 방식 모두 각자의 장단점이 있다. 가상 DOM은 선언적이고 예측 가능한 반면, 시그널은 더 효율적이고 세밀한 제어가 가능하다.

### 더 나아가기

이 구현에서 다루지 않은 고급 주제들:

- **Fiber 아키텍처**: 작업 분할과 우선순위 스케줄링
- **Concurrent Mode**: 중단 가능한 렌더링
- **Server Components**: 서버에서 실행되는 컴포넌트
- **Compiler Optimization**: Svelte처럼 빌드 타임 최적화

실제 프레임워크의 소스코드를 읽어보는 것을 추천한다:

- [Didact - Build your own React](https://github.com/pomber/didact)
- [Solid.js 소스코드](https://github.com/solidjs/solid/blob/main/packages/solid/src/reactive/signal.ts)
- [Preact 소스코드](https://github.com/preactjs/preact) (경량 React 대안)

직접 구현해보면서 얻은 이해는 어떤 문서보다 깊고 오래 남는다. 이 글이 렌더링의 본질을 이해하는 데 도움이 되었기를 바란다.
