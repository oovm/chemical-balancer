/**
 * 化学式AST节点类型
 */
export interface ChemicalElement {
    type: 'element';
    symbol: string; // 元素符号，如 'H', 'O', 'Ph' (虚拟元素)
    count: number;  // 原子数量
}

export interface ChemicalGroup {
    type: 'group';
    elements: ChemicalNode[];
    count: number; // 组的系数
}

export interface ChemicalIon {
    type: 'ion';
    elements: ChemicalNode[];
    charge: number; // 电荷数，正数为正电荷，负数为负电荷
    count: number;  // 离子数量
}

export type ChemicalNode = ChemicalElement | ChemicalGroup | ChemicalIon;

export interface ChemicalCompound {
    nodes: ChemicalNode[];
    coefficient: number; // 化学式前的系数
}

export interface ChemicalEquation {
    reactants: ChemicalCompound[]; // 反应物
    products: ChemicalCompound[];  // 生成物
}

/**
 * 配平结果
 */
export interface BalanceResult {
    success: boolean;
    equations: ChemicalEquation[]; // 可能有多个配平结果
    error?: string;
}

/**
 * 元素计数映射
 */
export interface ElementCount {
    [element: string]: number;
}

/**
 * 矩阵行
 */
export type MatrixRow = number[];

/**
 * 矩阵
 */
export type Matrix = MatrixRow[];