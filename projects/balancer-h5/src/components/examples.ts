// 响应式数据
export interface ChemicalExample {
    name: string
    equation: string
}

export const chemicalExamples: ChemicalExample[] = [
    {
        name: '甲烷燃烧',
        equation: 'CH4 + O2 == CO2 + H2O'
    },
    {
        name: '氢气燃烧',
        equation: 'H2 + O2 == H2O'
    },
    {
        name: '氨气合成',
        equation: 'N2 + H2 == NH3'
    },
    {
        name: '高锰酸钾氧化',
        equation: '[MnO4]- + [H]+ + [Cl]- == [Mn]2+ + H2O + Cl2'
    },
    {
        name: '铝热反应',
        equation: 'Al + Fe2O3 == Al2O3 + Fe'
    },
    {
        name: '碳酸钙分解',
        equation: 'CaCO3 == CaO + CO2'
    }
]