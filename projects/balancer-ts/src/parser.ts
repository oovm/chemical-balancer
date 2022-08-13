import {ChemicalCompound, ChemicalElement, ChemicalEquation, ChemicalGroup, ChemicalIon, ChemicalNode} from './types';

/**
 * 化学式解析器
 */
export class ChemicalParser {
    private input: string = '';
    private position: number = 0;

    /**
     * 解析化学方程式
     */
    parseEquation(equation: string): ChemicalEquation {
        const parts = equation.split(/\s*[=→]\s*/);
        if (parts.length !== 2) {
            throw new Error('Invalid equation format');
        }

        const reactants = this.parseCompounds(parts[0]);
        const products = this.parseCompounds(parts[1]);

        return {reactants, products};
    }

    /**
     * 解析多个化合物（用+分隔）
     */
    private parseCompounds(compoundsStr: string): ChemicalCompound[] {
        return compoundsStr.split(/\s*\+\s*/).map(compound => this.parseCompound(compound.trim()));
    }

    /**
     * 解析单个化合物
     */
    parseCompound(formula: string): ChemicalCompound {
        this.input = formula.trim();
        this.position = 0;

        // 解析系数
        const coefficient = this.parseCoefficient();

        // 解析化学式主体
        const nodes: ChemicalNode[] = [];
        while (this.position < this.input.length) {
            nodes.push(this.parseNode());
        }

        return {nodes, coefficient};
    }

    /**
     * 解析系数
     */
    private parseCoefficient(): number {
        let coefficient = '';
        while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
            coefficient += this.input[this.position];
            this.position++;
        }
        return coefficient ? parseInt(coefficient) : 1;
    }

    /**
     * 解析节点（元素、组或离子）
     */
    private parseNode(): ChemicalNode {
        if (this.peek() === '(') {
            return this.parseGroup();
        } else if (this.peek() === '[') {
            return this.parseIon();
        } else {
            return this.parseElement();
        }
    }

    /**
     * 解析元素
     */
    private parseElement(): ChemicalElement {
        const symbol = this.parseElementSymbol();
        const count = this.parseNumber();
        return {type: 'element', symbol, count};
    }

    /**
     * 解析元素符号（支持虚拟元素如Ph）
     */
    private parseElementSymbol(): string {
        if (this.position >= this.input.length || !/[A-Z]/.test(this.input[this.position])) {
            throw new Error(`Expected element symbol at position ${this.position}`);
        }
        // First char is A-Z
        let symbol = this.input[this.position];
        this.position++;

        // Subsequent chars must be a-z for standard elements and common groups like Ph, Et
        while (this.position < this.input.length && /[a-z]/.test(this.input[this.position])) {
            symbol += this.input[this.position];
            this.position++;
        }

        // For most cases, [A-Z][a-z]* is sufficient.
        return symbol;
    }

    /**
     * 解析组 ()
     */
    private parseGroup(): ChemicalGroup {
        this.expect('(');
        const elements: ChemicalNode[] = [];

        while (this.peek() !== ')') {
            elements.push(this.parseNode());
        }

        this.expect(')');
        const count = this.parseNumber();

        return {type: 'group', elements, count};
    }

    /**
     * 解析离子 []
     */
    private parseIon(): ChemicalIon {
        this.expect('[');
        const elements: ChemicalNode[] = [];

        while (this.peek() !== ']') {
            elements.push(this.parseNode());
        }

        this.expect(']');

        // 解析电荷
        const charge = this.parseCharge();
        const count = this.parseNumber();

        return {type: 'ion', elements, charge, count};
    }

    /**
     * 解析电荷
     * Handles formats like: +2, -2, 2+, 2-, +, -
     */
    private parseCharge(): number {
        // Store initial position for backtracking
        const initialPosition = this.position;

        if (this.position >= this.input.length) {
            // No characters left to parse
            return 0;
        }

        let sign = 1;
        let chargeStr = '';
        let signIsPrefix = false;
        let signIsPostfix = false;
        let digitsFound = false;

        // 1. Check for a prefix sign (+ or -)
        if (this.peek() === '+') {
            sign = 1;
            this.position++;
            signIsPrefix = true;
        } else if (this.peek() === '-') {
            sign = -1;
            this.position++;
            signIsPrefix = true;
        }

        // 2. Read digits for charge magnitude
        const digitsStartPosition = this.position;
        while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
            chargeStr += this.input[this.position];
            this.position++;
        }
        digitsFound = chargeStr.length > 0;

        // 3. If no prefix sign was found AND digits were found, check for a postfix sign
        if (!signIsPrefix && digitsFound) {
            if (this.peek() === '+') {
                // sign remains 1 (or could be set explicitly: sign = 1;)
                this.position++;
                signIsPostfix = true;
            } else if (this.peek() === '-') {
                sign = -1; // Set sign to negative
                this.position++;
                signIsPostfix = true;
            }
        }

        if (digitsFound) {
            // Valid if:
            // - [sign][digits] (e.g., +2, -2) -> signIsPrefix is true
            // - [digits][sign] (e.g., 2-, 2+) -> signIsPostfix is true
            // - [digits] alone is NOT a charge in this context, it would be a subscript.
            //   So, if only digitsFound is true, but neither signIsPrefix nor signIsPostfix, it's not a charge.
            if (signIsPrefix || signIsPostfix) {
                return sign * parseInt(chargeStr);
            } else {
                // Only digits found, no sign. This is not a charge. Backtrack.
                this.position = initialPosition;
                return 0;
            }
        } else {
            // No digits found
            // Valid if:
            // - [sign] (e.g., +, -) -> signIsPrefix is true
            if (signIsPrefix) {
                return sign * 1; // Charge is +/-1
            } else {
                // No digits and no prefix sign (e.g. empty string, or non-charge character)
                // Backtrack, as nothing was consumed that forms a charge.
                this.position = initialPosition;
                return 0;
            }
        }
    }

    /**
     * 解析数字
     */
    private parseNumber(): number {
        let numStr = '';
        while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
            numStr += this.input[this.position];
            this.position++;
        }
        return numStr ? parseInt(numStr) : 1;
    }

    /**
     * 查看当前字符
     */
    private peek(): string {
        return this.position < this.input.length ? this.input[this.position] : '';
    }

    /**
     * 期望特定字符
     */
    private expect(char: string): void {
        if (this.peek() !== char) {
            throw new Error(`Expected '${char}' at position ${this.position}, got '${this.peek()}'`);
        }
        this.position++;
    }
}