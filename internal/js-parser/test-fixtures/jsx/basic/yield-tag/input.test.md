# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `jsx > basic > yield-tag`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "jsx/basic/yield-tag/input.jsx"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array ["jsx"]
	loc: Object {
		filename: "jsx/basic/yield-tag/input.jsx"
		end: Object {
			column: 0
			line: 4
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "it"
				loc: Object {
					filename: "jsx/basic/yield-tag/input.jsx"
					identifierName: "it"
					end: Object {
						column: 11
						line: 1
					}
					start: Object {
						column: 9
						line: 1
					}
				}
			}
			loc: Object {
				filename: "jsx/basic/yield-tag/input.jsx"
				end: Object {
					column: 1
					line: 3
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: Array []
				rest: undefined
				returnType: undefined
				thisType: undefined
				typeParameters: undefined
				loc: Object {
					filename: "jsx/basic/yield-tag/input.jsx"
					end: Object {
						column: 13
						line: 1
					}
					start: Object {
						column: 11
						line: 1
					}
				}
			}
			body: JSBlockStatement {
				directives: Array []
				loc: Object {
					filename: "jsx/basic/yield-tag/input.jsx"
					end: Object {
						column: 1
						line: 3
					}
					start: Object {
						column: 13
						line: 1
					}
				}
				body: Array [
					JSExpressionStatement {
						loc: Object {
							filename: "jsx/basic/yield-tag/input.jsx"
							end: Object {
								column: 18
								line: 2
							}
							start: Object {
								column: 4
								line: 2
							}
						}
						expression: JSYieldExpression {
							delegate: false
							loc: Object {
								filename: "jsx/basic/yield-tag/input.jsx"
								end: Object {
									column: 17
									line: 2
								}
								start: Object {
									column: 4
									line: 2
								}
							}
							argument: JSXElement {
								name: JSXIdentifier {
									name: "a"
									loc: Object {
										filename: "jsx/basic/yield-tag/input.jsx"
										end: Object {
											column: 12
											line: 2
										}
										start: Object {
											column: 11
											line: 2
										}
									}
								}
								attributes: Array []
								children: Array []
								selfClosing: false
								typeArguments: undefined
								loc: Object {
									filename: "jsx/basic/yield-tag/input.jsx"
									end: Object {
										column: 17
										line: 2
									}
									start: Object {
										column: 10
										line: 2
									}
								}
							}
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```