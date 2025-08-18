---
title: '웹 렌더링의 진화: 명령형에서 선언적 함수형까지'
description: '웹 개발에서 DOM 렌더링 방식이 어떻게 진화해왔는지 살펴봅니다. 바닐라 JS부터 React까지, 각 시대의 문제점과 해결책, 그리고 가상 DOM을 사용하지 않는 최신 접근법까지 탐구합니다.'
date: '2025-08-18'
category: 'Render'
tags: ['DOM', 'Render']
readTime: '30분 읽기'
---

# 렌더링이란 무엇일까?

해당 주제를 말하기 전에 렌더링이 무엇인지부터 알고 갈 필요가 있어 보인다.

> **렌더링(rendering)** 또는 이미지 합성은 3D 모델과 같은 입력 데이터로부터 사실적인 또는 비사실적인 이미지를 생성하는 과정이다.
>
> "렌더링"이라는 단어는 원래 예술가가 실제 또는 상상의 것을 묘사할 때 수행하는 작업을 의미하며(완성된 예술 작품 또한 "렌더링"이라고 불림), 오늘날에는 일반적으로 컴퓨터 프로그램을 사용하여 정밀한 설명으로부터 이미지나 비디오를 생성하는 것을 의미한다.
>
> *— 위키피디아, "렌더링" 항목*

위키피디아의 정의를 살펴보면, 웹 브라우저에서의 렌더링은 HTML과 CSS를 통해 개발자가 설계한 내용을 실제 화면에 시각적으로 표현하는 과정이라고 할 수 있다.

## 브라우저는 어떻게 렌더링을 할까?

브라우저가 웹 페이지를 렌더링하는 과정을 이해하면, 왜 함수형 렌더링이 중요한지 더 명확해진다. 브라우저는 다음과 같은 단계를 거쳐 화면에 내용을 그려낸다.

### 1. HTML 파싱과 DOM 트리 생성

브라우저가 HTML 문서를 받으면, 가장 먼저 하는 일은 텍스트로 된 HTML을 파싱하여 DOM(Document Object Model) 트리를 생성하는 것이다.

```html
<!DOCTYPE html>
<html>
<head>
  <title>Todo App</title>
</head>
<body>
  <div id="app">
    <h1>할 일 목록</h1>
    <ul id="todo-list">
      <li>React 공부하기</li>
      <li>블로그 글 쓰기</li>
    </ul>
  </div>
</body>
</html>
```

이 HTML은 다음과 같은 DOM 트리로 변환된다:

```
html
├── head
│   └── title
│       └── "Todo App"
└── body
    └── div (id="app")
        ├── h1
        │   └── "할 일 목록"
        └── ul (id="todo-list")
            ├── li
            │   └── "React 공부하기"
            └── li
                └── "블로그 글 쓰기"
```

### 2. CSS 파싱과 CSSOM 생성

HTML과 동시에 CSS도 파싱되어 CSSOM(CSS Object Model)이 생성된다.

```css
#app {
  max-width: 600px;
  margin: 0 auto;
}

h1 {
  color: #333;
  font-size: 24px;
}

#todo-list {
  list-style: none;
  padding: 0;
}

li {
  padding: 10px;
  border-bottom: 1px solid #eee;
}
```

### 3. 렌더 트리 구성

DOM 트리와 CSSOM이 결합되어 렌더 트리(Render Tree)가 만들어진다. 렌더 트리는 화면에 실제로 표시될 요소들만 포함한다.

```
div (id="app")
├── h1 (color: #333, font-size: 24px)
└── ul (list-style: none, padding: 0)
    ├── li (padding: 10px, border-bottom: 1px solid #eee)
    └── li (padding: 10px, border-bottom: 1px solid #eee)
```

### 4. 레이아웃 (Layout/Reflow)

렌더 트리가 완성되면, 각 요소의 정확한 위치와 크기를 계산하는 레이아웃 과정이 시작된다.

```
div#app (x: 100px, y: 0px, width: 600px, height: 200px)
├── h1 (x: 100px, y: 0px, width: 600px, height: 50px)
└── ul (x: 100px, y: 50px, width: 600px, height: 150px)
    ├── li (x: 100px, y: 50px, width: 600px, height: 42px)
    └── li (x: 100px, y: 92px, width: 600px, height: 42px)
```

### 5. 페인팅 (Painting/Compositing)

마지막으로 계산된 정보를 바탕으로 실제 픽셀을 화면에 그리는 페인팅 과정이 진행된다.

```javascript
// 브라우저 내부에서 일어나는 페인팅 과정 (개념적 표현)
function paintElement(element, context) {
  context.fillStyle = element.backgroundColor;
  context.fillRect(element.x, element.y, element.width, element.height);
  
  context.fillStyle = element.textColor;
  context.fillText(element.text, element.x, element.y);
  
  if (element.border) {
    context.strokeRect(element.x, element.y, element.width, element.height);
  }
}
```

## 전통적인 DOM 조작의 문제점

브라우저 렌더링 과정을 이해했으니, 전통적인 DOM 조작 방식이 왜 비효율적인지 살펴보자.

### 리플로우와 리페인트의 비용

DOM을 직접 조작할 때마다 브라우저는 렌더링 파이프라인을 다시 실행해야 한다. 특히 레이아웃에 영향을 주는 변경사항은 비용이 크다.

```javascript
// 비효율적인 DOM 조작
const todoList = document.getElementById('todo-list');

// 각 조작마다 레이아웃 재계산 발생
todoList.style.display = 'none';  // 리플로우 발생
todoList.innerHTML = '';          // 리플로우 발생
todoList.appendChild(newItem1);   // 리플로우 발생
todoList.appendChild(newItem2);   // 리플로우 발생
todoList.appendChild(newItem3);   // 리플로우 발생
todoList.style.display = 'block'; // 리플로우 발생
```

위 코드는 총 6번의 리플로우를 발생시킨다. 리플로우는 요소의 크기나 위치를 다시 계산하는 과정으로, 페이지 전체에 영향을 줄 수 있어 성능상 비용이 크다.

#### 리플로우 vs 리페인트 성능 분석

브라우저 렌더링 비용을 구체적으로 분석해보자:

```javascript
// 성능 측정 도구
class RenderingProfiler {
  constructor() {
    this.metrics = {
      reflows: 0,
      repaints: 0,
      totalTime: 0
    };
  }
  
  measureDOMOperation(operation, description) {
    const startTime = performance.now();
    
    // 리플로우를 강제로 발생시키는 속성 읽기
    const heightBefore = document.body.offsetHeight;
    
    operation(); // DOM 조작 실행
    
    // 다시 레이아웃 정보 읽기 (리플로우 발생)
    const heightAfter = document.body.offsetHeight;
    
    const endTime = performance.now();
    const timeTaken = endTime - startTime;
    
    console.log(`${description}: ${timeTaken.toFixed(2)}ms`);
    
    if (heightBefore !== heightAfter) {
      this.metrics.reflows++;
    }
    
    this.metrics.totalTime += timeTaken;
    return timeTaken;
  }
}

// 성능 비교 테스트
const profiler = new RenderingProfiler();

// 1. 비효율적인 방식 (여러 번의 리플로우)
profiler.measureDOMOperation(() => {
  for (let i = 0; i < 100; i++) {
    const div = document.createElement('div');
    div.textContent = `Item ${i}`;
    div.style.padding = '10px'; // 각각 리플로우 발생
    document.body.appendChild(div);
  }
}, '개별 추가 (비효율적)');

// 2. 효율적인 방식 (배치 처리)
profiler.measureDOMOperation(() => {
  const fragment = document.createDocumentFragment();
  for (let i = 0; i < 100; i++) {
    const div = document.createElement('div');
    div.textContent = `Item ${i}`;
    div.style.padding = '10px';
    fragment.appendChild(div); // 메모리에서만 조작
  }
  document.body.appendChild(fragment); // 한 번만 리플로우
}, '배치 추가 (효율적)');

// 성능 차이: 개별 추가는 보통 5-10배 더 느림
```

#### 레이어 최적화와 복합 레이어

모던 브라우저는 GPU 가속을 위해 복합 레이어를 사용한다:

```css
/* GPU 레이어를 생성하는 CSS 속성들 */
.gpu-accelerated {
  /* 3D 변환 */
  transform: translateZ(0); /* 또는 translate3d(0,0,0) */
  
  /* 불투명도 애니메이션 */
  opacity: 0.99;
  
  /* 필터 효과 */
  filter: blur(0px);
  
  /* will-change로 명시적 레이어 생성 */
  will-change: transform, opacity;
}

/* 레이어 최적화를 활용한 애니메이션 */
.optimized-animation {
  /* 레이아웃에 영향을 주지 않는 속성만 사용 */
  transform: translateX(100px); /* 레이어에서만 처리 */
  opacity: 0.5;                 /* 레이어에서만 처리 */
}

.bad-animation {
  /* 레이아웃에 영향을 주는 속성 (피해야 함) */
  left: 100px;     /* 리플로우 발생 */
  width: 200px;    /* 리플로우 발생 */
  height: 150px;   /* 리플로우 발생 */
}
```

### DOM 조작의 예측하기 어려운 부작용

```javascript
// 어디서 todos 배열이 변경되는지 추적하기 어려움
let todos = [];

function addTodo(text) {
  todos.push({ id: Date.now(), text, completed: false });
  renderTodoList(); // UI 업데이트를 수동으로 호출해야 함
}

function markCompleted(id) {
  const todo = todos.find(t => t.id === id);
  todo.completed = true;
  // renderTodoList() 호출 깜빡함! UI가 업데이트되지 않음
}
```

#### 상태 동기화 문제의 심화 분석

전통적인 DOM 조작에서 발생하는 더 복잡한 문제들:

```javascript
// 복잡한 상태 관리 시나리오
class TodoManager {
  constructor() {
    this.todos = [];
    this.filters = { status: 'all', search: '' };
    this.ui = {
      todoList: document.getElementById('todo-list'),
      counter: document.getElementById('counter'),
      searchInput: document.getElementById('search'),
      filterButtons: document.querySelectorAll('.filter-btn')
    };
  }
  
  // 문제: 여러 UI 요소가 같은 상태에 의존
  updateUI() {
    this.renderTodoList();
    this.updateCounter();
    this.updateFilterButtons();
    this.updateSearchResults();
  }
  
  addTodo(text) {
    this.todos.push({ id: Date.now(), text, completed: false });
    // 어떤 UI 요소들을 업데이트해야 하는지 기억해야 함
    this.renderTodoList();
    this.updateCounter();
    // 검색 결과 업데이트를 깜빡함!
  }
  
  // 문제: 부분 업데이트 시 일관성 보장이 어려움
  toggleTodo(id) {
    const todo = this.todos.find(t => t.id === id);
    todo.completed = !todo.completed;
    
    // 개별 DOM 노드만 업데이트 시도
    const element = document.getElementById(`todo-${id}`);
    element.classList.toggle('completed');
    
    // 하지만 카운터는 업데이트 안 됨 - 일관성 깨짐
    // this.updateCounter(); // 깜빡함!
  }
  
  // 문제: 비동기 업데이트 시 경쟁 상태
  async searchTodos(query) {
    this.ui.searchInput.disabled = true;
    
    const results = await this.fetchSearchResults(query);
    
    // 사용자가 더 빠르게 타이핑한 경우 이전 결과가 나중에 도착할 수 있음
    this.todos = results;
    this.renderTodoList(); // 잘못된 결과 표시 가능
    
    this.ui.searchInput.disabled = false;
  }
}
```

### 메모리 누수 위험과 자원 관리

```javascript
// 메모리 누수 패턴들
class LeakyTodoManager {
  constructor() {
    this.todos = [];
    this.timers = new Set();
    this.eventListeners = new Map();
  }
  
  // 문제 1: 이벤트 리스너 정리 누락
  addTodoWithAutoSave(text) {
    const todo = { id: Date.now(), text, completed: false };
    this.todos.push(todo);
    
    const element = this.createTodoElement(todo);
    
    // 이벤트 리스너 추가하지만 정리 로직 없음
    const clickHandler = () => this.toggleTodo(todo.id);
    element.addEventListener('click', clickHandler);
    
    // 자동 저장 타이머 설정하지만 정리 안 함
    const saveTimer = setInterval(() => {
      this.saveTodo(todo);
    }, 5000);
    
    this.timers.add(saveTimer);
    this.eventListeners.set(element, clickHandler);
  }
  
  // 문제 2: DOM 요소 제거 시 관련 자원 정리 안 함
  removeTodo(id) {
    this.todos = this.todos.filter(t => t.id !== id);
    
    const element = document.getElementById(`todo-${id}`);
    element.remove(); // DOM만 제거
    
    // 타이머와 이벤트 리스너는 계속 실행됨 - 메모리 누수!
  }
  
  // 올바른 정리 방법
  dispose() {
    // 모든 타이머 정리
    this.timers.forEach(timer => clearInterval(timer));
    this.timers.clear();
    
    // 모든 이벤트 리스너 정리
    this.eventListeners.forEach((handler, element) => {
      element.removeEventListener('click', handler);
    });
    this.eventListeners.clear();
  }
}

// 메모리 사용량 모니터링
class MemoryMonitor {
  static measureMemoryUsage(operation, description) {
    if (!performance.memory) {
      console.log('Memory monitoring not supported');
      return;
    }
    
    const before = performance.memory.usedJSHeapSize;
    
    operation();
    
    // 가비지 컬렉션 강제 실행 (개발 환경에서만)
    if (window.gc) {
      window.gc();
    }
    
    setTimeout(() => {
      const after = performance.memory.usedJSHeapSize;
      const diff = after - before;
      
      console.log(`${description}:`);
      console.log(`메모리 변화: ${(diff / 1024 / 1024).toFixed(2)} MB`);
      console.log(`총 사용량: ${(after / 1024 / 1024).toFixed(2)} MB`);
    }, 100);
  }
}

// 사용 예시
MemoryMonitor.measureMemoryUsage(() => {
  const leakyManager = new LeakyTodoManager();
  
  // 1000개의 투두 아이템 추가 (메모리 누수 발생)
  for (let i = 0; i < 1000; i++) {
    leakyManager.addTodoWithAutoSave(`Todo ${i}`);
  }
}, '메모리 누수 테스트');
```

## 웹 렌더링 방식의 진화

이러한 DOM 조작의 문제점들을 해결하기 위해 웹 개발에서는 다양한 렌더링 방식들이 발전해왔다. 각 시대별로 어떤 문제를 해결하고 새로운 한계를 만들어 왔는지 살펴보자.

### 1단계: 바닐라 JavaScript 시대 (2000년대 초~중반)

초기 웹 개발에서는 직접 DOM API를 사용하여 요소를 조작했다:

```javascript
// 전통적인 바닐라 JavaScript 방식
function addTodoItem(text) {
  const todoList = document.getElementById('todo-list');
  const listItem = document.createElement('li');
  listItem.textContent = text;
  
  const deleteButton = document.createElement('button');
  deleteButton.textContent = 'Delete';
  deleteButton.onclick = function() {
    todoList.removeChild(listItem);
  };
  
  listItem.appendChild(deleteButton);
  todoList.appendChild(listItem);
}
```

**이 방식의 문제점:**
- DOM을 직접 조작하므로 코드가 복잡해짐
- 상태와 UI가 분리되어 동기화 문제 발생
- 이벤트 리스너 관리의 어려움
- 재사용 가능한 컴포넌트 개념 부재

### 2단계: jQuery 시대 (2006~2015)

jQuery는 DOM 조작을 더 쉽게 만들어줬지만 근본적인 문제는 해결하지 못했다:

```javascript
// jQuery를 사용한 Todo 리스트 (핵심 부분만)
let todos = [];

function renderTodos() {
  const $todoList = $('#todo-list');
  $todoList.empty(); // 전체 리스트 비우기
  
  todos.forEach(function(todo, index) {
    const $listItem = $('<li>')
      .addClass(todo.completed ? 'completed' : '')
      .html(`<span>${todo.text}</span>
             <button data-index="${index}">Delete</button>`);
    $todoList.append($listItem);
  });
}

// 상태 변경 시마다 전체 재렌더링
function addTodo(text) {
  todos.push({ text: text, completed: false });
  renderTodos(); // 전체 다시 그리기
}
```

**jQuery의 개선점:**
- 더 간결한 DOM 조작 문법 (`$()` 함수)
- 크로스 브라우저 호환성 해결
- 이벤트 위임을 통한 동적 요소 처리

**여전한 문제점:**
- 작은 변화에도 전체 리스트 재렌더링
- 상태와 UI 수동 동기화
- 성능 최적화의 한계

### 3단계: Angular.js 시대 (2010~2016)

Angular.js는 **양방향 데이터 바인딩**과 **더티 체킹**을 통해 렌더링을 자동화했다:

```javascript
// Angular.js Controller
app.controller('TodoController', function($scope) {
  $scope.todos = [];
  $scope.newTodo = '';
  
  $scope.addTodo = function() {
    if ($scope.newTodo.trim()) {
      $scope.todos.push({ text: $scope.newTodo, completed: false });
      $scope.newTodo = '';
    }
  };
});
```

```html
<!-- Angular.js 템플릿 -->
<div ng-controller="TodoController">
  <input ng-model="newTodo" placeholder="Add a todo...">
  <button ng-click="addTodo()">Add</button>
  
  <ul>
    <li ng-repeat="todo in todos">
      <span>{{todo.text}}</span>
    </li>
  </ul>
</div>
```

Angular.js의 핵심인 더티 체킹은 모든 바인딩된 값들을 주기적으로 확인하여 변경 사항을 감지한다:

```javascript
// 더티 체킹 메커니즘 (단순화)
function digestCycle() {
  let dirty;
  do {
    dirty = false;
    watchers.forEach(watcher => {
      const newValue = watcher.watchExpression();
      if (newValue !== watcher.lastValue) {
        watcher.listener(newValue, watcher.lastValue);
        watcher.lastValue = newValue;
        dirty = true; // 변화가 있으면 다시 체크
      }
    });
  } while (dirty); // 더 이상 변화가 없을 때까지 반복
}
```

**Angular.js의 혁신:**
- 자동 렌더링 (상태 변경 시 UI 자동 업데이트)
- 선언적 템플릿 (HTML에 직접 로직 작성)
- 양방향 바인딩 (모델과 뷰의 자동 동기화)

**한계점:**
- 더티 체킹으로 인한 성능 이슈 (모든 값을 반복 확인)
- 대규모 애플리케이션에서 예측하기 어려운 동작
- 언제 렌더링이 발생하는지 추적 곤란

### 4단계: React와 함수형 렌더링 시대 (2013~현재)

React는 **함수형 렌더링**과 **가상 DOM**을 통해 렌더링 패러다임을 완전히 바꿨다:

```jsx
// React를 사용한 Todo 리스트
import React, { useState } from 'react';

function TodoApp() {
  const [todos, setTodos] = useState([]);
  const [newTodo, setNewTodo] = useState('');
  
  const addTodo = () => {
    if (newTodo.trim()) {
      setTodos([...todos, { 
        id: Date.now(), 
        text: newTodo, 
        completed: false 
      }]);
      setNewTodo('');
    }
  };
  
  // 순수 함수로서의 렌더링 - 같은 상태는 항상 같은 결과
  return (
    <div>
      <input 
        value={newTodo}
        onChange={(e) => setNewTodo(e.target.value)}
        placeholder="Add a todo..."
      />
      <button onClick={addTodo}>Add</button>
      
      <ul>
        {todos.map(todo => (
          <TodoItem key={todo.id} todo={todo} />
        ))}
      </ul>
    </div>
  );
}

// 컴포넌트 분리로 재사용성 향상
function TodoItem({ todo }) {
  return (
    <li className={todo.completed ? 'completed' : ''}>
      <span>{todo.text}</span>
    </li>
  );
}
```

**React의 핵심 혁신:**
- **순수 함수 기반 렌더링**: 상태 → UI 변환이 예측 가능
- **가상 DOM**: 실제 DOM 조작 최소화로 성능 향상
- **컴포넌트 기반 아키텍처**: 재사용 가능한 UI 단위
- **단방향 데이터 흐름**: 데이터 흐름이 명확하고 추적 가능

### 진화의 핵심: 명령형에서 선언적으로

핵심 변화는 **명령형**에서 **선언적** 프로그래밍으로의 전환이다:

```javascript
// 명령형: "어떻게 할 것인가?"
function addTodoImperative(text) {
  const li = document.createElement('li');
  li.textContent = text;
  li.addEventListener('click', handleClick);
  todoList.appendChild(li);
}

// 선언적: "무엇을 보여줄 것인가?"
function TodoList({ todos }) {
  return (
    <ul>
      {todos.map(todo => 
        <TodoItem key={todo.id} todo={todo} />
      )}
    </ul>
  );
}
```

각 시대는 이전 방식의 한계를 극복하면서 개발자 경험과 성능을 동시에 개선해왔다.

## 함수형 렌더링의 핵심 개념

React로 대표되는 함수형 렌더링이 왜 혁신적인지 구체적으로 살펴보자.

### 1. 예측 가능성 (Predictability)

이전 방식들에서는 상태와 UI가 분리되어 있어 동기화 문제가 발생했다:

```javascript
// jQuery - 상태와 UI가 분리되어 있음
let todos = []; // 상태
$('#todo-list').html('...'); // UI 조작

// 언제 어디서 todos가 변경되는지 추적하기 어려움
function someFunction() {
  todos.push(newItem); // 상태 변경
  // UI는 여전히 이전 상태를 보여줌!
}
```

React는 이 문제를 함수형 접근으로 해결했다:

```jsx
// 상태 → UI 변환이 순수 함수
function TodoList({ todos }) {
  // 같은 todos 배열은 항상 같은 JSX를 반환
  return (
    <ul>
      {todos.map(todo => <li key={todo.id}>{todo.text}</li>)}
    </ul>
  );
}

// 같은 입력 → 같은 출력 보장
const todos = [{ text: 'React 학습', completed: false }];
console.log(renderTodoList(todos)); // 항상 같은 결과
```

### 2. 테스트 용이성

순수 함수는 테스트하기 쉽다:

```javascript
// 순수 함수는 테스트하기 쉽다
test('completed todo should have completed class', () => {
  const todos = [{ text: 'Test', completed: true }];
  const result = renderTodoList(todos);
  expect(result).toContain('class="completed"');
});
```

### 3. 가상 DOM을 통한 성능 최적화

함수형 렌더링의 핵심 아이디어는 **상태를 UI로 변환하는 순수 함수**를 만들되, 성능 문제를 가상 DOM으로 해결하는 것이다:

```javascript
// React 내부 동작 (단순화된 버전)
function reconcile(prevElement, nextElement, container) {
  // 1. 타입이 다르면 완전 교체
  if (prevElement.type !== nextElement.type) {
    container.replaceChild(
      createElement(nextElement),
      container.firstChild
    );
    return;
  }
  
  // 2. 같은 타입이면 props와 children만 비교
  updateProps(container.firstChild, prevElement.props, nextElement.props);
  
  // 3. 자식 요소들 재귀적으로 비교
  reconcileChildren(
    container.firstChild,
    prevElement.children,
    nextElement.children
  );
}

// 실제 DOM 변경은 최소한으로만 발생
function updateProps(element, prevProps, nextProps) {
  // 제거된 props
  Object.keys(prevProps).forEach(key => {
    if (!(key in nextProps)) {
      element.removeAttribute(key);
    }
  });
  
  // 추가/변경된 props만 업데이트
  Object.keys(nextProps).forEach(key => {
    if (prevProps[key] !== nextProps[key]) {
      element.setAttribute(key, nextProps[key]);
    }
  });
}
```

가상 DOM은 변경점을 효율적으로 계산하여 실제 DOM 조작을 최소화한다.

## React Fiber: 차세대 렌더링 아키텍처

React의 가상 DOM을 구현하는 핵심 기술이 바로 **Fiber**다. Fiber는 React 16에서 도입된 새로운 재조정(reconciliation) 알고리즘으로, 렌더링 성능과 사용자 경험을 획기적으로 개선했다.

### Fiber 이전의 한계점

React 15까지는 **스택 기반 재조정**을 사용했다. 이 방식은 렌더링이 시작되면 중단할 수 없어 문제가 있었다:

```javascript
// React 15 스타일의 재조정 (단순화)
function reconcileChildren(element, children) {
  // 재귀적으로 모든 자식을 처리 - 중단 불가능
  children.forEach(child => {
    if (child.type === 'component') {
      const instance = new child.type(child.props);
      const newChildren = instance.render();
      reconcileChildren(child, newChildren); // 깊이 우선 탐색
    } else {
      updateDOMNode(child);
    }
  });
}

// 문제: 큰 컴포넌트 트리에서 16ms 초과하면 프레임 드롭 발생
function bigComponentTree() {
  return (
    <div>
      {Array.from({ length: 10000 }, (_, i) => (
        <ExpensiveComponent key={i} />
      ))}
    </div>
  ); // 이 전체가 한 번에 처리되어야 함
}
```

### Fiber의 핵심 개념

Fiber는 **작업을 작은 단위로 나누어 우선순위에 따라 스케줄링**하는 아키텍처다:

```javascript
// Fiber 노드 구조 (단순화)
interface FiberNode {
  // 컴포넌트 정보
  type: string | Function;           // 'div', Component 등
  props: any;                        // 속성들
  stateNode: any;                    // 실제 DOM 노드 또는 인스턴스
  
  // 트리 구조
  child: FiberNode | null;           // 첫 번째 자식
  sibling: FiberNode | null;         // 다음 형제
  return: FiberNode | null;          // 부모 (return은 예약어가 아님)
  
  // 작업 단위
  alternate: FiberNode | null;       // 이전 버전의 Fiber (더블 버퍼링)
  effectTag: EffectTag;              // 수행할 작업 타입
  updateQueue: UpdateQueue | null;   // 상태 업데이트 큐
  
  // 스케줄링
  lanes: Lanes;                      // 우선순위 정보
  childLanes: Lanes;                 // 자식들의 우선순위
}

type EffectTag = 
  | 'NoEffect'     // 변경 없음
  | 'Placement'    // 새로 추가
  | 'Update'       // 속성 변경
  | 'Deletion'     // 제거
  | 'ChildDeletion'; // 자식 제거
```

### Fiber의 작업 단위 분할

Fiber는 렌더링을 두 단계로 나눈다:

#### 1. 렌더 단계 (Render Phase) - 중단 가능

```javascript
// 작업 단위별로 처리하여 중단 가능
function workLoopConcurrent() {
  while (workInProgress !== null && !shouldYield()) {
    workInProgress = performUnitOfWork(workInProgress);
  }
}

function performUnitOfWork(unitOfWork: FiberNode): FiberNode | null {
  const current = unitOfWork.alternate;
  
  // 1. 현재 노드 작업 수행
  let next = beginWork(current, unitOfWork, renderLanes);
  
  if (next === null) {
    // 2. 자식이 없으면 완료 작업 수행
    completeUnitOfWork(unitOfWork);
  }
  
  return next;
}

function shouldYield(): boolean {
  // 브라우저가 다른 작업을 해야 하는지 확인
  return getCurrentTime() >= deadline;
}
```

#### 2. 커밋 단계 (Commit Phase) - 중단 불가능

```javascript
// DOM 업데이트는 동기적으로 한 번에 처리
function commitRoot(root: FiberRoot) {
  const finishedWork = root.finishedWork;
  
  // 1. DOM 변경 전 작업 (getSnapshotBeforeUpdate 등)
  commitBeforeMutationEffects(finishedWork);
  
  // 2. 실제 DOM 변경
  commitMutationEffects(finishedWork);
  
  // 3. DOM 변경 후 작업 (componentDidUpdate 등)
  commitLayoutEffects(finishedWork);
}
```

### Fiber의 순회 알고리즘

Fiber는 **깊이 우선 탐색**을 사용하되, 각 노드에서 중단할 수 있다:

```javascript
// Fiber 트리 순회 (단순화)
function traverseFiberTree(fiber: FiberNode) {
  let node = fiber;
  
  while (node) {
    // 1. 현재 노드 처리
    console.log('Processing:', node.type);
    
    // 2. 자식이 있으면 자식으로 이동
    if (node.child) {
      node = node.child;
      continue;
    }
    
    // 3. 자식이 없으면 완료 처리 후 형제로 이동
    while (node) {
      console.log('Completing:', node.type);
      
      if (node.sibling) {
        node = node.sibling;
        break;
      }
      
      // 형제도 없으면 부모로 올라가기
      node = node.return;
    }
  }
}

// 예시 트리 구조:
//     App
//    /   \
//   div   span
//  /
// h1
//
// 순회 순서: App → div → h1 → (h1 완료) → (div 완료) → span → (span 완료) → (App 완료)
```

### 차이점 계산 알고리즘 (Diffing)

Fiber는 효율적인 차이점 계산을 위해 몇 가지 휴리스틱을 사용한다:

#### 1. 타입 기반 비교

```javascript
function reconcileChildren(
  current: FiberNode | null,
  workInProgress: FiberNode,
  nextChildren: ReactElement
) {
  if (current === null) {
    // 초기 마운트
    workInProgress.child = mountChildFibers(workInProgress, null, nextChildren);
  } else {
    // 업데이트
    workInProgress.child = reconcileChildFibers(
      workInProgress,
      current.child,
      nextChildren
    );
  }
}

function reconcileSingleElement(
  returnFiber: FiberNode,
  currentFirstChild: FiberNode | null,
  element: ReactElement
): FiberNode {
  const key = element.key;
  let child = currentFirstChild;
  
  while (child !== null) {
    // key와 type이 모두 같으면 재사용
    if (child.key === key && child.type === element.type) {
      // 재사용 가능한 Fiber 발견
      const existing = useFiber(child, element.props);
      existing.return = returnFiber;
      
      // 나머지 형제들은 삭제
      deleteRemainingChildren(returnFiber, child.sibling);
      return existing;
    }
    
    // 재사용 불가능하면 삭제 표시
    deleteChild(returnFiber, child);
    child = child.sibling;
  }
  
  // 새로운 Fiber 생성
  const created = createFiberFromElement(element);
  created.return = returnFiber;
  return created;
}
```

#### 2. 리스트 비교 알고리즘

```javascript
function reconcileChildrenArray(
  returnFiber: FiberNode,
  currentFirstChild: FiberNode | null,
  newChildren: ReactElement[]
): FiberNode | null {
  let resultingFirstChild: FiberNode | null = null;
  let previousNewFiber: FiberNode | null = null;
  let oldFiber = currentFirstChild;
  let newIdx = 0;
  
  // 1단계: 같은 위치에서 key와 type이 같은 것들 처리
  for (; oldFiber !== null && newIdx < newChildren.length; newIdx++) {
    if (oldFiber.index > newIdx) break;
    
    const newFiber = updateSlot(returnFiber, oldFiber, newChildren[newIdx]);
    if (newFiber === null) break; // key가 다르면 중단
    
    // 새로운 fiber를 리스트에 연결
    if (previousNewFiber === null) {
      resultingFirstChild = newFiber;
    } else {
      previousNewFiber.sibling = newFiber;
    }
    previousNewFiber = newFiber;
    oldFiber = oldFiber.sibling;
  }
  
  // 2단계: 새로운 요소들이 더 있으면 추가
  if (newIdx === newChildren.length) {
    deleteRemainingChildren(returnFiber, oldFiber);
    return resultingFirstChild;
  }
  
  // 3단계: 기존 요소들이 더 있으면 Map으로 관리
  if (oldFiber === null) {
    // 모든 새 요소들 추가
    for (; newIdx < newChildren.length; newIdx++) {
      const newFiber = createChild(returnFiber, newChildren[newIdx]);
      if (newFiber === null) continue;
      
      if (previousNewFiber === null) {
        resultingFirstChild = newFiber;
      } else {
        previousNewFiber.sibling = newFiber;
      }
      previousNewFiber = newFiber;
    }
    return resultingFirstChild;
  }
  
  // 4단계: 복잡한 경우 - key 기반 Map 사용
  const existingChildren = mapRemainingChildren(returnFiber, oldFiber);
  
  for (; newIdx < newChildren.length; newIdx++) {
    const newFiber = updateFromMap(
      existingChildren,
      returnFiber,
      newIdx,
      newChildren[newIdx]
    );
    
    if (newFiber !== null) {
      if (newFiber.alternate !== null) {
        // 재사용된 fiber는 Map에서 제거
        existingChildren.delete(newFiber.key === null ? newIdx : newFiber.key);
      }
      
      if (previousNewFiber === null) {
        resultingFirstChild = newFiber;
      } else {
        previousNewFiber.sibling = newFiber;
      }
      previousNewFiber = newFiber;
    }
  }
  
  // 5단계: Map에 남은 것들은 모두 삭제
  existingChildren.forEach(child => deleteChild(returnFiber, child));
  
  return resultingFirstChild;
}
```

### Lane 기반 우선순위 시스템

React 18에서는 **Lane** 모델을 사용하여 업데이트 우선순위를 관리한다:

```javascript
// Lane 우선순위 (비트마스크)
const SyncLane = 0b0000000000000000000000000000001;
const InputContinuousLane = 0b0000000000000000000000000000100;
const DefaultLane = 0b0000000000000000000000000010000;
const TransitionLane1 = 0b0000000000000000000000001000000;
const IdleLane = 0b0100000000000000000000000000000;

function scheduleUpdateOnFiber(fiber: FiberNode, lane: Lane) {
  // 우선순위에 따라 스케줄링
  if (lane === SyncLane) {
    // 동기 업데이트 (onClick 등)
    performSyncWorkOnRoot(root);
  } else {
    // 비동기 업데이트 (setTimeout, Promise 등)
    ensureRootIsScheduled(root, getCurrentTime());
  }
}

function getNextLanes(root: FiberRoot, wipLanes: Lanes): Lanes {
  const pendingLanes = root.pendingLanes;
  
  if (pendingLanes === NoLanes) {
    return NoLanes;
  }
  
  // 가장 높은 우선순위 lane 찾기
  const nextLanes = getHighestPriorityLanes(pendingLanes);
  
  // Concurrent 모드에서는 낮은 우선순위 작업도 굶지 않도록 관리
  if (wipLanes !== NoLanes && wipLanes !== nextLanes) {
    const nextLane = getHighestPriorityLane(nextLanes);
    const wipLane = getHighestPriorityLane(wipLanes);
    
    // 현재 작업 중인 우선순위가 더 높거나 같으면 계속 진행
    if (nextLane >= wipLane) {
      return wipLanes;
    }
  }
  
  return nextLanes;
}
```

### Time Slicing과 Concurrent Features

Fiber의 핵심 기능인 **Time Slicing**:

```javascript
// Scheduler의 작업 분할
function workLoopConcurrent() {
  while (workInProgress !== null && !shouldYield()) {
    workInProgress = performUnitOfWork(workInProgress);
  }
}

function shouldYield(): boolean {
  const timeElapsed = getCurrentTime() - startTime;
  
  // 5ms 이상 작업했으면 양보
  if (timeElapsed < 5) {
    return false;
  }
  
  // 브라우저의 다른 작업 확인
  if (navigator.scheduling?.isInputPending()) {
    return true;
  }
  
  return timeElapsed >= frameYieldMs;
}

// Concurrent Features 예시
function App() {
  const [isPending, startTransition] = useTransition();
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  
  const handleSearch = (value) => {
    setQuery(value); // 높은 우선순위 (즉시 반영)
    
    startTransition(() => {
      // 낮은 우선순위 (백그라운드에서 처리)
      setResults(expensiveSearch(value));
    });
  };
  
  return (
    <div>
      <input value={query} onChange={e => handleSearch(e.target.value)} />
      {isPending && <div>검색 중...</div>}
      <SearchResults results={results} />
    </div>
  );
}
```

### Fiber의 성능 이점

```javascript
// 성능 비교 시뮬레이션
function measurePerformance() {
  const startTime = performance.now();
  
  // React 15 스타일 (블로킹)
  function oldReconciliation() {
    // 전체 트리를 한 번에 처리
    for (let i = 0; i < 10000; i++) {
      processComponent(components[i]);
    }
    // 16ms 초과 시 프레임 드롭 발생
  }
  
  // Fiber 스타일 (논블로킹)
  function fiberReconciliation() {
    let processed = 0;
    
    function workLoop() {
      const deadline = performance.now() + 5; // 5ms 타임 슬라이스
      
      while (processed < 10000 && performance.now() < deadline) {
        processComponent(components[processed]);
        processed++;
      }
      
      if (processed < 10000) {
        // 다음 프레임에서 계속
        requestIdleCallback(workLoop);
      }
    }
    
    workLoop();
  }
  
  const endTime = performance.now();
  console.log(`처리 시간: ${endTime - startTime}ms`);
}
```

### Fiber 알고리즘의 시간 복잡도 분석

Fiber의 성능을 수학적으로 분석해보자:

```javascript
// 알고리즘 복잡도 분석
class FiberComplexityAnalyzer {
  // O(n) - 각 노드를 한 번씩만 방문
  static analyzeFiberTraversal(fiberTree) {
    let nodeCount = 0;
    let depth = 0;
    let maxDepth = 0;
    
    function traverse(fiber, currentDepth = 0) {
      if (!fiber) return;
      
      nodeCount++;
      depth = Math.max(depth, currentDepth);
      
      // 자식 순회 (깊이 우선)
      if (fiber.child) {
        traverse(fiber.child, currentDepth + 1);
      }
      
      // 형제 순회 (같은 레벨)
      if (fiber.sibling) {
        traverse(fiber.sibling, currentDepth);
      }
    }
    
    traverse(fiberTree);
    
    return {
      nodeCount,      // O(n) 노드 수
      maxDepth: depth, // O(log n) 균형 트리에서
      complexity: `O(${nodeCount})` // 선형 시간
    };
  }
  
  // 차이점 계산 복잡도: O(n + m) where n = old tree size, m = new tree size
  static analyzeReconciliation(oldTree, newTree) {
    let comparisons = 0;
    let creations = 0;
    let deletions = 0;
    let updates = 0;
    
    function diff(oldFiber, newElement, index = 0) {
      comparisons++;
      
      if (!oldFiber && !newElement) {
        return null; // 둘 다 없음
      }
      
      if (!oldFiber && newElement) {
        creations++;
        return { type: 'CREATE', element: newElement };
      }
      
      if (oldFiber && !newElement) {
        deletions++;
        return { type: 'DELETE', fiber: oldFiber };
      }
      
      // 타입이 다르면 교체 (O(1))
      if (oldFiber.type !== newElement.type) {
        deletions++;
        creations++;
        return { type: 'REPLACE', oldFiber, newElement };
      }
      
      // 같은 타입이면 업데이트 (O(k) where k = props count)
      if (this.hasPropsChanged(oldFiber.props, newElement.props)) {
        updates++;
        return { type: 'UPDATE', fiber: oldFiber, newProps: newElement.props };
      }
      
      return null; // 변경 없음
    }
    
    return {
      comparisons,
      creations,
      deletions,
      updates,
      totalOperations: comparisons + creations + deletions + updates
    };
  }
}
```

### Fiber 스케줄러의 고급 알고리즘

```javascript
// React Scheduler의 내부 구현 (단순화)
class ReactScheduler {
  constructor() {
    this.taskQueue = new MinHeap(); // 우선순위 큐
    this.timerQueue = new MinHeap(); // 지연된 작업 큐
    this.currentTask = null;
    this.currentPriorityLevel = NormalPriority;
    this.isSchedulerPaused = false;
  }
  
  // 우선순위 기반 작업 스케줄링
  scheduleCallback(priorityLevel, callback, options = {}) {
    const currentTime = getCurrentTime();
    let startTime = currentTime;
    
    if (options.delay && options.delay > 0) {
      startTime = currentTime + options.delay;
    }
    
    let timeout;
    switch (priorityLevel) {
      case ImmediatePriority:
        timeout = IMMEDIATE_PRIORITY_TIMEOUT; // -1 (즉시 실행)
        break;
      case UserBlockingPriority:
        timeout = USER_BLOCKING_PRIORITY_TIMEOUT; // 250ms
        break;
      case IdlePriority:
        timeout = IDLE_PRIORITY_TIMEOUT; // 1073741823ms (거의 무한대)
        break;
      case LowPriority:
        timeout = LOW_PRIORITY_TIMEOUT; // 10000ms
        break;
      case NormalPriority:
      default:
        timeout = NORMAL_PRIORITY_TIMEOUT; // 5000ms
        break;
    }
    
    const expirationTime = startTime + timeout;
    
    const newTask = {
      id: taskIdCounter++,
      callback,
      priorityLevel,
      startTime,
      expirationTime,
      sortIndex: -1,
    };
    
    if (startTime > currentTime) {
      // 지연된 작업
      newTask.sortIndex = startTime;
      this.timerQueue.push(newTask);
      this.requestHostTimeout(this.handleTimeout, startTime - currentTime);
    } else {
      // 즉시 실행 가능한 작업
      newTask.sortIndex = expirationTime;
      this.taskQueue.push(newTask);
      this.requestHostCallback(this.flushWork);
    }
    
    return newTask;
  }
  
  // 작업 실행 루프
  flushWork(hasTimeRemaining, initialTime) {
    this.isSchedulerPaused = false;
    
    // 지연된 작업들을 태스크 큐로 이동
    this.advanceTimers(initialTime);
    
    try {
      return this.workLoop(hasTimeRemaining, initialTime);
    } finally {
      this.currentTask = null;
      this.currentPriorityLevel = NormalPriority;
      
      // 남은 작업이 있으면 다음 스케줄링
      if (this.taskQueue.peek() !== null) {
        this.requestHostCallback(this.flushWork);
      } else {
        const firstTimer = this.timerQueue.peek();
        if (firstTimer !== null) {
          this.requestHostTimeout(
            this.handleTimeout,
            firstTimer.startTime - getCurrentTime()
          );
        }
      }
    }
  }
  
  workLoop(hasTimeRemaining, initialTime) {
    let currentTime = initialTime;
    this.advanceTimers(currentTime);
    
    this.currentTask = this.taskQueue.peek();
    
    while (this.currentTask !== null && !this.isSchedulerPaused) {
      if (
        this.currentTask.expirationTime > currentTime &&
        (!hasTimeRemaining || this.shouldYieldToHost())
      ) {
        // 시간이 부족하거나 양보해야 함
        break;
      }
      
      const callback = this.currentTask.callback;
      if (typeof callback === 'function') {
        this.currentTask.callback = null;
        this.currentPriorityLevel = this.currentTask.priorityLevel;
        
        const didUserCallbackTimeout = this.currentTask.expirationTime <= currentTime;
        const continuationCallback = callback(didUserCallbackTimeout);
        
        currentTime = getCurrentTime();
        
        if (typeof continuationCallback === 'function') {
          // 작업이 완료되지 않음 - 계속 실행
          this.currentTask.callback = continuationCallback;
        } else {
          // 작업 완료 - 큐에서 제거
          if (this.currentTask === this.taskQueue.peek()) {
            this.taskQueue.pop();
          }
        }
        
        this.advanceTimers(currentTime);
      } else {
        this.taskQueue.pop();
      }
      
      this.currentTask = this.taskQueue.peek();
    }
    
    // 더 실행할 작업이 있으면 true 반환
    return this.currentTask !== null;
  }
  
  shouldYieldToHost() {
    const timeElapsed = getCurrentTime() - this.startTime;
    
    // 5ms 규칙: 5ms 이상 실행했으면 양보
    if (timeElapsed < 5) {
      return false;
    }
    
    // 브라우저 API를 사용한 더 정확한 체크
    if (navigator.scheduling?.isInputPending()) {
      return true;
    }
    
    return timeElapsed >= this.yieldInterval;
  }
}

// 우선순위 큐 구현 (Min Heap)
class MinHeap {
  constructor() {
    this.heap = [];
  }
  
  peek() {
    return this.heap[0] || null;
  }
  
  push(node) {
    this.heap.push(node);
    this.siftUp(this.heap.length - 1);
  }
  
  pop() {
    if (this.heap.length === 0) return null;
    
    const first = this.heap[0];
    const last = this.heap.pop();
    
    if (this.heap.length > 0) {
      this.heap[0] = last;
      this.siftDown(0);
    }
    
    return first;
  }
  
  siftUp(index) {
    while (index > 0) {
      const parentIndex = Math.floor((index - 1) / 2);
      
      if (this.compare(this.heap[index], this.heap[parentIndex]) >= 0) {
        break;
      }
      
      this.swap(index, parentIndex);
      index = parentIndex;
    }
  }
  
  siftDown(index) {
    while (true) {
      let minIndex = index;
      const leftChild = 2 * index + 1;
      const rightChild = 2 * index + 2;
      
      if (
        leftChild < this.heap.length &&
        this.compare(this.heap[leftChild], this.heap[minIndex]) < 0
      ) {
        minIndex = leftChild;
      }
      
      if (
        rightChild < this.heap.length &&
        this.compare(this.heap[rightChild], this.heap[minIndex]) < 0
      ) {
        minIndex = rightChild;
      }
      
      if (minIndex === index) break;
      
      this.swap(index, minIndex);
      index = minIndex;
    }
  }
  
  compare(a, b) {
    return a.sortIndex - b.sortIndex;
  }
  
  swap(i, j) {
    [this.heap[i], this.heap[j]] = [this.heap[j], this.heap[i]];
  }
}
```

### Concurrent Mode의 고급 기능

```javascript
// Suspense와 Error Boundary를 활용한 고급 렌더링
class AdvancedRenderingPatterns {
  // Suspense for Data Fetching
  static createSuspenseDataFetcher() {
    const cache = new Map();
    
    function fetchData(url) {
      if (cache.has(url)) {
        const cached = cache.get(url);
        
        if (cached.status === 'fulfilled') {
          return cached.value;
        }
        
        if (cached.status === 'rejected') {
          throw cached.reason;
        }
        
        // 아직 pending 상태
        throw cached.promise;
      }
      
      // 새로운 요청 생성
      const promise = fetch(url)
        .then(response => response.json())
        .then(
          data => {
            cache.set(url, { status: 'fulfilled', value: data });
            return data;
          },
          error => {
            cache.set(url, { status: 'rejected', reason: error });
            throw error;
          }
        );
      
      cache.set(url, { status: 'pending', promise });
      throw promise; // Suspense에게 대기 상태 알림
    }
    
    return fetchData;
  }
  
  // Time Slicing을 활용한 대용량 리스트 렌더링
  static createTimeSlicedRenderer() {
    return function TimeSlicedList({ items, renderItem }) {
      const [displayCount, setDisplayCount] = useState(50);
      const [isRendering, setIsRendering] = useState(false);
      
      useEffect(() => {
        if (displayCount < items.length) {
          setIsRendering(true);
          
          // 낮은 우선순위로 추가 렌더링 스케줄
          startTransition(() => {
            setTimeout(() => {
              setDisplayCount(count => Math.min(count + 50, items.length));
              setIsRendering(false);
            }, 0);
          });
        }
      }, [displayCount, items.length]);
      
      return (
        <div>
          {items.slice(0, displayCount).map(renderItem)}
          {isRendering && <div>더 많은 항목을 로딩 중...</div>}
        </div>
      );
    };
  }
  
  // 선택적 하이드레이션 패턴
  static createSelectiveHydration() {
    const hydratedComponents = new Set();
    
    function withSelectiveHydration(Component, componentId) {
      return function SelectivelyHydratedComponent(props) {
        const [isHydrated, setIsHydrated] = useState(
          hydratedComponents.has(componentId)
        );
        
        useEffect(() => {
          // 뷰포트에 들어오거나 사용자 상호작용 시 하이드레이션
          const observer = new IntersectionObserver(
            (entries) => {
              if (entries[0].isIntersecting && !isHydrated) {
                setIsHydrated(true);
                hydratedComponents.add(componentId);
              }
            },
            { threshold: 0.1 }
          );
          
          const element = document.getElementById(componentId);
          if (element) {
            observer.observe(element);
          }
          
          return () => observer.disconnect();
        }, [isHydrated]);
        
        if (!isHydrated) {
          // SSR된 정적 HTML 유지
          return <div id={componentId} suppressHydrationWarning />;
        }
        
        return <Component {...props} />;
      };
    }
    
    return withSelectiveHydration;
  }
}
```

Fiber 아키텍처는 React가 **부드러운 사용자 경험**과 **복잡한 애플리케이션 성능**을 동시에 달성할 수 있게 해주는 핵심 기술이다. 작업 분할, 우선순위 스케줄링, 효율적인 차이점 계산, 그리고 고급 Concurrent 기능들을 통해 현대 웹 애플리케이션의 요구사항을 충족시킨다.

## 가상 DOM을 사용하지 않는 대안적 접근법

가상 DOM이 효과적인 해결책이지만, 모든 상황에 최적은 아니다. 최근에는 가상 DOM 없이도 효율적인 렌더링을 구현하는 다양한 방법들이 등장했다.

### 컴파일 타임 최적화 (Compile-time Optimization)

Svelte와 같은 프레임워크는 빌드 시점에 코드를 분석하여 최적화된 DOM 조작 코드를 생성한다.

```svelte
<!-- Svelte 컴포넌트 -->
<script>
  let todos = [];
  let newTodo = '';
  
  function addTodo() {
    if (newTodo.trim()) {
      todos = [...todos, { id: Date.now(), text: newTodo, completed: false }];
      newTodo = '';
    }
  }
</script>

<input bind:value={newTodo} placeholder="Add a todo..." />
<button on:click={addTodo}>Add</button>

<ul>
  {#each todos as todo (todo.id)}
    <li class:completed={todo.completed}>
      {todo.text}
    </li>
  {/each}
</ul>
```

Svelte 컴파일러는 이 코드를 다음과 같은 최적화된 JavaScript로 변환한다:

```javascript
// Svelte가 생성하는 코드 (단순화)
function update_todos(changed) {
  if (changed.todos) {
    const each_blocks = [];
    
    for (let i = 0; i < todos.length; i++) {
      each_blocks[i] = create_todo_block(todos[i]);
    }
    
    // 변경된 부분만 DOM 업데이트
    for (let i = 0; i < each_blocks.length; i++) {
      each_blocks[i].mount(ul_element);
    }
  }
}
```

**컴파일 최적화의 장점:**
- 런타임 오버헤드 최소화 (가상 DOM 없음)
- 번들 크기 감소 (프레임워크 런타임 불필요)
- 컴파일 시점 최적화로 뛰어난 성능

### Proxy 기반 반응성 (Proxy-based Reactivity)

Vue 3과 Solid.js는 Proxy를 사용하여 상태 변경을 감지하고 필요한 부분만 업데이트한다.

#### Vue 3의 반응성 시스템

```javascript
// Vue 3의 reactive 시스템
import { reactive, effect } from 'vue';

const state = reactive({
  todos: [],
  filter: 'all'
});

// 상태 변경을 자동으로 감지
effect(() => {
  // todos나 filter가 변경되면 자동 실행
  const filteredTodos = state.todos.filter(todo => {
    if (state.filter === 'completed') return todo.completed;
    if (state.filter === 'pending') return !todo.completed;
    return true;
  });
  
  updateTodoList(filteredTodos);
});

// 상태 변경 시 effect 자동 실행
state.todos.push({ id: 1, text: 'Learn Vue 3', completed: false });
```

#### Solid.js의 Fine-grained Reactivity

```javascript
// Solid.js 스타일의 시그널 구현
function createSignal(initialValue) {
  let value = initialValue;
  const subscribers = new Set();

  const read = () => {
    // 현재 실행 중인 effect가 있다면 구독자로 등록
    if (currentEffect) {
      subscribers.add(currentEffect);
    }
    return value;
  };

  const write = (newValue) => {
    if (typeof newValue === 'function') {
      newValue = newValue(value);
    }

    if (value !== newValue) {
      value = newValue;
      // 모든 구독자들에게 변경 알림
      subscribers.forEach(effect => effect.execute());
    }
  };

  return [read, write];
}

function createEffect(fn) {
  const effect = {
    execute() {
      const prevEffect = currentEffect;
      currentEffect = effect;
      try {
        fn();
      } finally {
        currentEffect = prevEffect;
      }
    }
  };

  effect.execute(); // 초기 실행
  return effect;
}

let currentEffect = null;

// 사용 예시
function TodoApp() {
  const [todos, setTodos] = createSignal([]);
  const [filter, setFilter] = createSignal('all');

  // 필터링된 todos를 계산하는 derived signal
  const filteredTodos = () => {
    const currentTodos = todos();
    const currentFilter = filter();

    return currentTodos.filter(todo => {
      if (currentFilter === 'completed') return todo.completed;
      if (currentFilter === 'pending') return !todo.completed;
      return true;
    });
  };

  // DOM 업데이트 effect
  createEffect(() => {
    const todoList = document.getElementById('todo-list');
    const currentTodos = filteredTodos();

    // 직접 DOM 조작 - 가상 DOM 없음
    todoList.innerHTML = '';
    currentTodos.forEach(todo => {
      const li = document.createElement('li');
      li.textContent = todo.text;
      li.className = todo.completed ? 'completed' : '';
      todoList.appendChild(li);
    });
  });

  return {
    addTodo: (text) => setTodos(prev => [...prev, { id: Date.now(), text, completed: false }]),
    toggleTodo: (id) => setTodos(prev => prev.map(todo =>
      todo.id === id ? { ...todo, completed: !todo.completed } : todo
    )),
    setFilter
  };
}
```

**Proxy 기반 반응성의 장점:**
- 세밀한 업데이트 (변경된 값만 정확히 업데이트)
- 가상 DOM 오버헤드 없음
- 자동 의존성 추적
- 뛰어난 런타임 성능

### 세 가지 접근법의 심화 비교

각 렌더링 방식의 기술적 특성을 더 자세히 분석해보자:

#### 1. 가상 DOM 방식 (React)

```javascript
// React의 렌더링 프로세스 분석
class ReactRenderingAnalysis {
  static measureVirtualDOMOverhead() {
    const componentTree = {
      type: 'div',
      props: {},
      children: Array.from({ length: 1000 }, (_, i) => ({
        type: 'span',
        props: { key: i },
        children: [`Item ${i}`]
      }))
    };
    
    console.time('Virtual DOM Creation');
    const vdom = this.createVirtualDOM(componentTree);
    console.timeEnd('Virtual DOM Creation');
    
    console.time('Virtual DOM Diffing');
    const patches = this.diffVirtualDOM(vdom, vdom); // 같은 트리 비교
    console.timeEnd('Virtual DOM Diffing');
    
    return {
      vdomSize: this.calculateVDOMSize(vdom),
      patchCount: patches.length,
      memoryUsage: this.estimateMemoryUsage(vdom)
    };
  }
  
  // 메모리 사용량 추정
  static estimateMemoryUsage(vdom) {
    let size = 0;
    
    function traverse(node) {
      if (typeof node === 'string') {
        size += node.length * 2; // UTF-16 문자열
        return;
      }
      
      size += 100; // 객체 오버헤드 (추정)
      size += JSON.stringify(node.props).length * 2;
      
      if (node.children) {
        node.children.forEach(traverse);
      }
    }
    
    traverse(vdom);
    return size;
  }
}
```

#### 2. 컴파일 타임 최적화 (Svelte)

```javascript
// Svelte 컴파일러 시뮬레이션
class SvelteCompilerSimulation {
  // 의존성 분석 알고리즘
  static analyzeDependencies(component) {
    const dependencies = new Map();
    const ast = this.parseComponent(component);
    
    // 1. 반응형 변수 탐지
    const reactiveVars = this.findReactiveVariables(ast);
    
    // 2. DOM 조작 지점 식별
    const domMutations = this.findDOMMutations(ast);
    
    // 3. 의존성 그래프 구축
    reactiveVars.forEach(variable => {
      const dependents = domMutations.filter(mutation => 
        mutation.usedVariables.includes(variable)
      );
      dependencies.set(variable, dependents);
    });
    
    return dependencies;
  }
  
  // 컴파일된 업데이트 함수 생성
  static generateUpdateFunction(dependencies) {
    const updateBlocks = [];
    
    dependencies.forEach((mutations, variable) => {
      const updateBlock = `
        if (changed.${variable}) {
          ${mutations.map(m => m.updateCode).join('\n')}
        }
      `;
      updateBlocks.push(updateBlock);
    });
    
    return `
      function update(changed) {
        ${updateBlocks.join('\n')}
      }
    `;
  }
  
  // 성능 비교: 런타임 vs 컴파일타임
  static comparePerformance() {
    const startTime = performance.now();
    
    // 런타임 방식 (React 스타일)
    for (let i = 0; i < 1000; i++) {
      this.runtimeUpdate(); // 매번 diff 계산
    }
    const runtimeTime = performance.now() - startTime;
    
    const compileStartTime = performance.now();
    
    // 컴파일타임 방식 (Svelte 스타일)
    for (let i = 0; i < 1000; i++) {
      this.compiledUpdate(); // 사전 생성된 최적 코드
    }
    const compileTime = performance.now() - compileStartTime;
    
    return {
      runtime: runtimeTime,
      compiled: compileTime,
      improvement: ((runtimeTime - compileTime) / runtimeTime * 100).toFixed(1)
    };
  }
}
```

#### 3. Proxy 기반 반응성 (Vue 3, Solid.js)

```javascript
// 고급 Proxy 기반 반응성 시스템
class AdvancedReactivitySystem {
  constructor() {
    this.effects = new Set();
    this.targetMap = new WeakMap();
    this.activeEffect = null;
    this.effectStack = [];
  }
  
  // 중첩된 객체의 깊은 반응성
  reactive(target) {
    if (this.isReactive(target)) {
      return target;
    }
    
    return this.createReactiveObject(target, this.mutableHandlers);
  }
  
  createReactiveObject(target, handlers) {
    const proxy = new Proxy(target, handlers);
    this.markReactive(proxy);
    return proxy;
  }
  
  // 최적화된 Proxy 핸들러
  get mutableHandlers() {
    return {
      get: (target, key, receiver) => {
        // 의존성 추적
        this.track(target, 'get', key);
        
        const result = Reflect.get(target, key, receiver);
        
        // 중첩된 객체도 반응형으로 변환
        if (this.isObject(result)) {
          return this.reactive(result);
        }
        
        return result;
      },
      
      set: (target, key, value, receiver) => {
        const oldValue = target[key];
        const result = Reflect.set(target, key, value, receiver);
        
        // 값이 실제로 변경된 경우에만 트리거
        if (this.hasChanged(value, oldValue)) {
          this.trigger(target, 'set', key, value, oldValue);
        }
        
        return result;
      },
      
      deleteProperty: (target, key) => {
        const hadKey = Object.prototype.hasOwnProperty.call(target, key);
        const result = Reflect.deleteProperty(target, key);
        
        if (result && hadKey) {
          this.trigger(target, 'delete', key);
        }
        
        return result;
      }
    };
  }
  
  // 스마트 의존성 추적
  track(target, type, key) {
    if (!this.activeEffect) return;
    
    let depsMap = this.targetMap.get(target);
    if (!depsMap) {
      this.targetMap.set(target, (depsMap = new Map()));
    }
    
    let dep = depsMap.get(key);
    if (!dep) {
      depsMap.set(key, (dep = new Set()));
    }
    
    if (!dep.has(this.activeEffect)) {
      dep.add(this.activeEffect);
      this.activeEffect.deps.push(dep);
    }
  }
  
  // 배치 업데이트로 성능 최적화
  trigger(target, type, key, newValue, oldValue) {
    const depsMap = this.targetMap.get(target);
    if (!depsMap) return;
    
    const effects = new Set();
    const computedRunners = new Set();
    
    // 영향받는 이펙트들 수집
    if (key !== void 0) {
      this.addRunners(effects, computedRunners, depsMap.get(key));
    }
    
    // 배열의 length 변경 시 인덱스 관련 이펙트들도 트리거
    if (type === 'add' || type === 'delete') {
      this.addRunners(effects, computedRunners, depsMap.get('length'));
    }
    
    // 컴퓨티드 값들을 먼저 업데이트
    computedRunners.forEach(runner => runner());
    
    // 일반 이펙트들 실행
    effects.forEach(effect => effect());
  }
  
  // 성능 측정
  measureReactivityPerformance() {
    const state = this.reactive({
      count: 0,
      items: Array.from({ length: 1000 }, (_, i) => ({ id: i, value: i * 2 }))
    });
    
    let updateCount = 0;
    
    // 이펙트 등록
    this.effect(() => {
      // count와 첫 번째 아이템에 의존
      const sum = state.count + state.items[0].value;
      updateCount++;
    });
    
    console.time('Reactivity Updates');
    
    // 1000번의 업데이트
    for (let i = 0; i < 1000; i++) {
      state.count++; // 이펙트 트리거
      state.items[500].value++; // 이펙트 트리거 안 함 (의존성 없음)
    }
    
    console.timeEnd('Reactivity Updates');
    
    return {
      totalUpdates: 1000,
      effectTriggers: updateCount,
      efficiency: `${((1000 - updateCount + 1000) / 2000 * 100).toFixed(1)}%`
    };
  }
}
```

## 현재와 미래의 렌더링 트렌드

웹 개발의 렌더링 기술은 계속 진화하고 있다. 현재 주목받고 있는 새로운 접근법들을 살펴보자.

### 서버 사이드 렌더링의 부활

초기 웹의 서버 사이드 렌더링이 현대적인 형태로 돌아오고 있다:

```jsx
// Next.js 서버 컴포넌트
async function TodoList() {
  // 서버에서 데이터를 가져와서 HTML로 전송
  const todos = await fetchTodosFromDB();
  
  return (
    <ul>
      {todos.map(todo => <TodoItem key={todo.id} todo={todo} />)}
    </ul>
  );
}

// 클라이언트에서는 이미 렌더링된 HTML을 받아서 hydration만 수행
```

장점: 초기 로딩 속도 향상, SEO 개선, 자바스크립트 번들 크기 감소

### 점진적 hydration과 스트리밍

```jsx
// 컴포넌트별로 점진적 로딩
function App() {
  return (
    <div>
      <Header /> {/* 즉시 로딩 */}
      <Suspense fallback={<TodoListSkeleton />}>
        <TodoList /> {/* 데이터 준비되면 스트리밍 */}
      </Suspense>
      <Suspense fallback={<CommentsSkeleton />}>
        <Comments /> {/* 별도로 지연 로딩 */}
      </Suspense>
    </div>
  );
}
```

### 세밀한 반응성 (Fine-grained Reactivity)

React의 컴포넌트 단위 업데이트 대신, 변경된 값만 정확히 업데이트하는 방식:

```javascript
// Solid.js 스타일의 세밀한 반응성
function TodoItem() {
  const [todo, setTodo] = createSignal({ id: 1, text: 'Learn Solid', completed: false });
  
  return (
    <li>
      {/* text가 변경되면 이 텍스트 노드만 업데이트 */}
      <span>{todo().text}</span>
      {/* completed가 변경되면 이 버튼만 업데이트 */}
      <button>{todo().completed ? '완료' : '미완료'}</button>
    </li>
  );
}
```

이 방식은 컴포넌트 전체를 리렌더링하지 않고 변경된 부분만 업데이트하여 더 나은 성능을 제공한다.

## 마무리

웹 렌더링의 진화는 단순한 기술적 발전이 아니라 사고방식의 전환이다. 명령형에서 선언적으로, DOM 조작에서 상태 관리로, 그리고 이제는 컴파일 최적화와 세밀한 반응성으로 발전하고 있다.

### 핵심 교훈들

복잡도 관리의 중요성

- 초기 DOM 조작: O(n²) 복잡도와 예측 불가능한 부작용
- 가상 DOM: O(n) 복잡도와 예측 가능한 업데이트
- 컴파일러 최적화: O(1) 에 가까운 최적화된 업데이트

각각의 접근법은 고유한 장단점을 가지며, 프로젝트의 요구사항에 따라 적절한 선택이 필요하다. 중요한 것은 각 방식의 원리를 이해하고, 상황에 맞는 최적의 도구를 선택하는 것이다.

앞으로도 웹 렌더링 기술은 계속 진화할 것이다. WebAssembly, AI, GPU 가속 등 새로운 기술들이 렌더링 성능을 한층 더 향상시킬 것이며, 개발자는 이러한 변화의 흐름을 이해하고 적응해 나가는 것이 중요하다.

렌더링의 내부 동작을 완전히 이해하려면 직접 만들어보는 것이 가장 좋다. 가상 DOM 생성, 차이점 계산, DOM 업데이트, 반응성 시스템까지 포함한 완전한 렌더링 엔진을 구현하는 방법은 다음 편에서 자세히 다루도록 하겠다.
