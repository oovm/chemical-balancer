<template>
  <div class="space-y-6">
    <!-- 输入区域 -->
    <div class="card">
      <h2 class="text-xl font-semibold text-gray-800 mb-4">
        输入化学方程式
      </h2>
      
      <div class="space-y-4">
        <!-- 反应物输入 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            反应物 (用 + 分隔)
          </label>
          <input
            v-model="reactants"
            type="text"
            class="input-field w-full"
            placeholder="例如: CH4 + O2 或 [MnO4]- + [H]+ + [Cl]-"
            @keyup.enter="balanceEquation"
          />
        </div>

        <!-- 操作按钮 -->
        <div class="flex items-center justify-center space-x-4">
          <button
            @click="swapReactantsProducts"
            class="btn-secondary flex items-center space-x-2"
            title="交换反应物和生成物"
          >
            <i class="i-carbon-arrow-left-right text-lg"></i>
            <span>交换</span>
          </button>
          
          <div class="text-2xl text-gray-400">→</div>
          
          <button
            @click="balanceEquation"
            class="btn-primary flex items-center space-x-2"
            :disabled="isBalancing"
          >
            <i class="i-carbon-chemistry text-lg"></i>
            <span>{{ isBalancing ? '配平中...' : '配平' }}</span>
          </button>
        </div>

        <!-- 生成物输入 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            生成物 (用 + 分隔)
          </label>
          <input
            v-model="products"
            type="text"
            class="input-field w-full"
            placeholder="例如: CO2 + H2O 或 [Mn]2+ + H2O + Cl2"
            @keyup.enter="balanceEquation"
          />
        </div>
      </div>
    </div>

    <!-- 示例方程式 -->
    <div class="card">
      <h3 class="text-lg font-medium text-gray-800 mb-3">
        示例方程式
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <button
          v-for="example in examples"
          :key="example.name"
          @click="loadExample(example)"
          class="text-left p-3 rounded-lg border border-gray-200 hover:border-primary-300 hover:bg-primary-50 transition-colors"
        >
          <div class="font-medium text-gray-800">{{ example.name }}</div>
          <div class="text-sm text-gray-600 mt-1">{{ example.equation }}</div>
        </button>
      </div>
    </div>

    <!-- 结果显示 -->
    <div v-if="result" class="card">
      <h3 class="text-lg font-medium text-gray-800 mb-4">
        配平结果
      </h3>
      
      <div v-if="result.success" class="space-y-4">
        <!-- LaTeX 渲染结果 -->
        <div class="bg-gray-50 rounded-lg p-4">
          <div class="text-sm font-medium text-gray-700 mb-2">配平后的方程式:</div>
          <div 
            ref="latexContainer"
            class="text-center text-lg"
          ></div>
        </div>
        
        <!-- 纯文本结果 -->
        <div class="bg-blue-50 rounded-lg p-4">
          <div class="text-sm font-medium text-gray-700 mb-2">纯文本格式:</div>
          <div class="font-mono text-gray-800">{{ result.text }}</div>
        </div>
        
        <!-- HTML 结果 -->
        <div class="bg-green-50 rounded-lg p-4">
          <div class="text-sm font-medium text-gray-700 mb-2">HTML格式:</div>
          <div 
            class="text-gray-800"
            v-html="result.html"
          ></div>
        </div>
      </div>
      
      <div v-else class="bg-red-50 border border-red-200 rounded-lg p-4">
        <div class="flex items-center space-x-2">
          <i class="i-carbon-warning text-red-500"></i>
          <span class="font-medium text-red-800">配平失败</span>
        </div>
        <div class="text-red-700 mt-2">{{ result.error }}</div>
      </div>
    </div>

    <!-- 使用说明 -->
    <div class="card">
      <h3 class="text-lg font-medium text-gray-800 mb-3">
        使用说明
      </h3>
      <div class="space-y-2 text-sm text-gray-600">
        <p><strong>基本格式:</strong> 使用元素符号和数字，如 H2O、CH4</p>
        <p><strong>离子:</strong> 使用方括号表示，如 [SO4]2-、[H]+</p>
        <p><strong>虚拟元素:</strong> 支持如 Ph、Et 等有机基团</p>
        <p><strong>括号:</strong> 使用圆括号表示基团，如 Ca(OH)2</p>
        <p><strong>分隔符:</strong> 反应物和生成物用 + 分隔</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import { ChemicalEquationBalancer } from 'chemical-balancer-ts'
import katex from 'katex'

interface Example {
  name: string
  equation: string
  reactants: string
  products: string
}

const reactants = ref('')
const products = ref('')
const isBalancing = ref(false)
const result = ref<any>(null)
const latexContainer = ref<HTMLElement>()

const balancer = new ChemicalEquationBalancer()

const examples: Example[] = [
  {
    name: '甲烷燃烧',
    equation: 'CH4 + O2 → CO2 + H2O',
    reactants: 'CH4 + O2',
    products: 'CO2 + H2O'
  },
  {
    name: '高锰酸钾氧化',
    equation: 'KMnO4 + HCl → KCl + MnCl2 + H2O + Cl2',
    reactants: 'KMnO4 + HCl',
    products: 'KCl + MnCl2 + H2O + Cl2'
  },
  {
    name: '离子反应',
    equation: '[MnO4]- + [H]+ + [Cl]- → [Mn]2+ + H2O + Cl2',
    reactants: '[MnO4]- + [H]+ + [Cl]-',
    products: '[Mn]2+ + H2O + Cl2'
  },
  {
    name: '虚拟元素',
    equation: 'PhCH3 + O2 → CO2 + H2O',
    reactants: 'PhCH3 + O2',
    products: 'CO2 + H2O'
  },
  {
    name: '复杂化合物',
    equation: 'Al2(SO4)3 + Ca(OH)2 → Al(OH)3 + CaSO4',
    reactants: 'Al2(SO4)3 + Ca(OH)2',
    products: 'Al(OH)3 + CaSO4'
  },
  {
    name: '葡萄糖燃烧',
    equation: 'C6H12O6 + O2 → CO2 + H2O',
    reactants: 'C6H12O6 + O2',
    products: 'CO2 + H2O'
  }
]

const loadExample = (example: Example) => {
  reactants.value = example.reactants
  products.value = example.products
}

const swapReactantsProducts = () => {
  const temp = reactants.value
  reactants.value = products.value
  products.value = temp
}

const balanceEquation = async () => {
  if (!reactants.value.trim() || !products.value.trim()) {
    return
  }
  
  isBalancing.value = true
  result.value = null
  
  try {
    const equation = `${reactants.value.trim()} = ${products.value.trim()}`
    const balanceResult = balancer.balance(equation)
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
      error: error instanceof Error ? error.message : '未知错误'
    }
  } finally {
    isBalancing.value = false
  }
}

// 初始化示例
onMounted(() => {
  loadExample(examples[0])
})
</script>