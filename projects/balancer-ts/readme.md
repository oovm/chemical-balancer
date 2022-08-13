# chemical-balancer

A TypeScript library for balancing chemical equations.

## Overview
This package provides a robust solution to balance chemical equations programmatically. It handles various chemical species including elements, compounds, ions, and complex groups, leveraging `fraction.js` for precise fractional coefficient calculations.

## Installation
Install via npm or pnpm:

```bash
npm install chemical-balancer
# or
pnpm add chemical-balancer
```

## Basic Usage
Import the balancer and use it to balance equations:

```typescript
import { balanceEquation } from 'chemical-balancer';

// Balance H2 + O2 → H2O
const result = balanceEquation('H2 + O2 -> H2O');
console.log(result.balancedEquation); // '2H2 + O2 = 2H2O'
```

## API Documentation
### `balanceEquation(equation: string): BalanceResult`
- **Parameters**: A chemical equation string (e.g., 'Fe + H2O -> Fe3O4 + H2').
- **Returns**: An object containing `balancedEquation` (string) and `coefficients` (Record<string, number>).

## Features
- Supports ionic equations (e.g., 'Cu + AgNO3 -> Cu(NO3)2 + Ag').
- Handles polyatomic ions (e.g., SO4^2-, NH4+).
- Validates chemical formula syntax.
- Returns fractional coefficients if needed.
