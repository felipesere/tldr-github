module.exports = {
    roots: ["<rootDir>/src"],
    transform: {
        '^.+\\.tsx?$': 'ts-jest',
    },
    testRegex: '(/__tests__/.*|(\\.|/)(test|spec))\\.tsx?$',
    moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
    setupFilesAfterEnv: [
        "<rootDir>/test/setup.ts",
        "@testing-library/jest-dom/extend-expect"
    ],
};
