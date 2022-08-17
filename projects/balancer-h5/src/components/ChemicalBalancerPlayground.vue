<template>
  <div class="space-y-6">
    <!-- 输入区域 -->
    <div class="card">
      <h2 class="text-xl font-semibold text-gray-800 mb-4">
        {{ $t('chemical-balancer-playground-title') }}
      </h2>

      <div class="space-y-4">
        <!-- 化学方程式输入 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            {{ $t('chemical-equation-input-label') }}
          </label>
          <input
              v-model="equation"
              type="text"
              class="input-field w-full"
              :placeholder="$t('chemical-equation-input-placeholder')"
              @keyup.enter="balanceEquation"
          />
        </div>

        <!-- 操作按钮 -->
        <div class="flex items-center justify-center space-x-4">
          <button
              @click="swapReactantsProducts"
              class="btn-secondary flex items-center space-x-2"
              :title="$t('swap-button')"
          >
            <i class="i-carbon-arrow-left-right text-lg"></i>
            <span>{{ $t('swap-button') }}</span>
          </button>

          <button
              @click="balanceEquation"
              class="btn-primary flex items-center space-x-2"
              :disabled="isBalancing"
          >
            <i class="i-carbon-chemistry text-lg"></i>
            <span>{{ isBalancing ? $t('balancing-button') : $t('balance-button') }}</span>
          </button>

          <button
              @click="showExamples = true"
              class="btn-secondary flex items-center space-x-2"
          >
            <i class="i-carbon-list text-lg"></i>
            <span>{{ $t('examples-button') }}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- 示例方程式弹窗 -->
    <div v-if="showExamples" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
         @click="showExamples = false">
      <div class="bg-white rounded-xl shadow-xl max-w-2xl w-full mx-4 max-h-96 overflow-y-auto" @click.stop>
        <div class="p-6">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-medium text-gray-800">
              {{ $t('example-equations-title') }}
            </h3>
            <button @click="showExamples = false" class="text-gray-400 hover:text-gray-600">
              <i class="i-carbon-close text-xl"></i>
            </button>
          </div>
          <div class="grid grid-cols-1 gap-3">
            <button
                v-for="example in chemicalExamples"
                :key="example.name"
                @click="loadExample(example)"
                class="text-left p-3 rounded-lg border border-gray-200 hover:border-primary-300 hover:bg-primary-50 transition-colors"
            >
              <div class="font-medium text-gray-800">{{ example.name }}</div>
              <div class="text-sm text-gray-600 mt-1">{{ example.equation }}</div>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 结果显示 -->
    <div v-if="result" class="card">
      <h3 class="text-lg font-medium text-gray-800 mb-4">
        {{ $t('balancing-results-title') }}
      </h3>

      <div v-if="result.success" class="space-y-4">
        <!-- LaTeX 渲染结果 -->
        <div class="bg-gray-50 rounded-lg p-4">
          <div class="text-sm font-medium text-gray-700 mb-2">{{ $t('balanced-equation-label') }}</div>
          <div
              ref="latexContainer"
              class="text-center text-lg"
          ></div>
        </div>

        <!-- 纯文本结果 -->
        <div class="bg-blue-50 rounded-lg p-4">
          <div class="text-sm font-medium text-gray-700 mb-2">{{ $t('plain-text-format-label') }}</div>
          <div class="font-mono text-gray-800">{{ result.text }}</div>
        </div>

        <!-- HTML 结果 -->
        <div class="bg-green-50 rounded-lg p-4">
          <div class="text-sm font-medium text-gray-700 mb-2">{{ $t('html-format-label') }}</div>
          <div
              class="text-gray-800"
              v-html="result.html"
          ></div>
        </div>
      </div>

      <div v-else class="bg-red-50 border border-red-200 rounded-lg p-4">
        <div class="flex items-center space-x-2">
          <i class="i-carbon-warning text-red-500"></i>
          <span class="font-medium text-red-800">{{ $t('balancing-failed-title') }}</span>
        </div>
        <div class="text-red-700 mt-2">{{ result.error }}</div>
      </div>
    </div>

    <!-- 使用说明 -->
    <div class="card">
      <h3 class="text-lg font-medium text-gray-800 mb-3">
        {{ $t('usage-instructions-title') }}
      </h3>
      <div class="space-y-2 text-sm text-gray-600">
        <p><strong>{{ $t('basic-format-instruction').split(':')[0] }}:</strong> {{ $t('basic-format-instruction').split(':')[1] }}</p>
        <p><strong>{{ $t('ions-instruction').split(':')[0] }}:</strong> {{ $t('ions-instruction').split(':')[1] }}</p>
        <p><strong>{{ $t('virtual-elements-instruction').split(':')[0] }}:</strong> {{ $t('virtual-elements-instruction').split(':')[1] }}</p>
        <p><strong>{{ $t('parentheses-instruction').split(':')[0] }}:</strong> {{ $t('parentheses-instruction').split(':')[1] }}</p>
        <p><strong>{{ $t('separators-instruction').split(':')[0] }}:</strong> {{ $t('separators-instruction').split(':')[1] }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {nextTick, onMounted, ref} from 'vue'
import {ChemicalEquationBalancer} from 'chemical-balancer'
import katex from 'katex'
import {ChemicalExample, chemicalExamples} from "@/components/examples";
import {useFluent} from "fluent-vue";

const equation = ref('')
const isBalancing = ref(false)
const result = ref<any>(null)
const showExamples = ref(false)
const latexContainer = ref<HTMLElement>()
const { $t } = useFluent()

// 创建化学方程式配平器实例
const balancer = new ChemicalEquationBalancer()

const loadExample = (example: ChemicalExample) => {
  equation.value = example.equation
  showExamples.value = false
}

const swapReactantsProducts = () => {
  const parts = equation.value.split(/==|=/)
  if (parts.length === 2) {
    equation.value = `${parts[1].trim()} == ${parts[0].trim()}`
  }
}

const balanceEquation = async () => {
  if (!equation.value.trim()) {
    return
  }

  isBalancing.value = true
  result.value = null

  try {
    const balanceResult = balancer.balance(equation.value.trim())
    result.value = balanceResult

    // 渲染 LaTeX
    if (balanceResult.success && latexContainer.value) {
      await nextTick()
      try {
        katex.render(balanceResult.latex, latexContainer.value, {
          displayMode: true,
          throwOnError: false
        })
      } catch (error) {
        console.error('LaTeX rendering error:', error)
        if (latexContainer.value) {
          latexContainer.value.textContent = balanceResult.text
        }
      }
    }
  } catch (error) {
    result.value = {
      success: false,
      error: error instanceof Error ? error.message : $t('unknown-error')
    }
  } finally {
    isBalancing.value = false
  }
}

// 初始化示例
onMounted(() => {
  loadExample(chemicalExamples[0])
})

// 将 chemicalBalancer 实例暴露到 window 对象上
const chemicalBalancer = new ChemicalEquationBalancer()

// 暴露一个函数，用于在浏览器控制台或其他脚本中调用，获取 LaTeX 格式的配平结果
window.balanceEquationsLatex = (equation: string) => {
  try {
    const result = chemicalBalancer.balance(equation)
    if (result.success) {
      return result.latex
    } else {
      return `Error: ${result.error}`
    }
  } catch (e: any) {
    return `Error: ${e.message}`
  }
}

// 暴露一个函数，用于在浏览器控制台或其他脚本中调用，获取 JSON 格式的配平结果
window.balanceEquationsJson = (equation: string) => {
  try {
    const result = chemicalBalancer.balance(equation)
    return JSON.stringify(result, null, 2)
  } catch (e: any) {
    return JSON.stringify({ success: false, error: e.message }, null, 2)
  }
}
</script>