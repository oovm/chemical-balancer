/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'katex' {
  export function render(tex: string, element: HTMLElement, options?: any): void
  export function renderToString(tex: string, options?: any): string
}