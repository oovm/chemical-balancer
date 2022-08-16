<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
    <div class="container mx-auto px-4 py-8">
      <header class="text-center mb-8">
        <h1 class="text-4xl font-bold text-gray-800 mb-2">
          {{ $t('welcome') }}
        </h1>
      </header>

      <main class="max-w-4xl mx-auto">
        <ChemicalBalancerPlayground/>
      </main>

    </div>
  </div>
</template>

<script setup lang="ts">
import ChemicalBalancerPlayground from './components/ChemicalBalancerPlayground.vue'
</script>