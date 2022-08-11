<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
    <div class="container mx-auto px-4 py-8">
      <header class="text-center mb-8">
        <h1 class="text-4xl font-bold text-gray-800 mb-2">
          化学方程式配平器
        </h1>
        <p class="text-gray-600">
          支持离子配平、虚拟元素配平，使用零空间算法
        </p>
      </header>

      <main class="max-w-4xl mx-auto">
        <ChemicalBalancerPlayground />
      </main>

      <footer class="text-center mt-12 text-gray-500 text-sm">
        <p>基于零空间算法的化学方程式配平器</p>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import ChemicalBalancerPlayground from './components/ChemicalBalancerPlayground.vue'
</script>