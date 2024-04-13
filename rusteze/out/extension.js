"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
const vscode = __importStar(require("vscode"));
const spawn = require("child_process").spawn;
// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
function activate(context) {
    // Use the console to output diagnostic information (console.log) and errors (console.error)
    // This line of code will only be executed once when your extension is activated
    console.log('Congratulations, your extension "rusteze" is now active!');
    // The command has been defined in the package.json file
    // Now provide the implementation of the command with registerCommand
    // The commandId parameter must match the command field in package.json
    let disposable = vscode.commands.registerCommand('rusteze.rustify', async () => {
        // The code you place here will be executed every time your command is executed
        // Display a message box to the user
        // Get the active text editor
        const editor = vscode.window.activeTextEditor;
        if (editor) {
            const content = editor.document.getText();
            const wsedit = new vscode.WorkspaceEdit();
            const wsPath = vscode.workspace.workspaceFolders[0].uri.fsPath; // gets the path of the first workspace folder
            const filePath = vscode.Uri.file(wsPath + '/hello/world.md');
            vscode.window.showInformationMessage(filePath.toString());
            wsedit.createFile(filePath, { ignoreIfExists: true });
            vscode.workspace.applyEdit(wsedit);
            vscode.window.showInformationMessage('Created a new file: hello/world.md');
            const pythonProcess = await spawn('python3', ['Users/alexs/documents/mhacks24/c2rust/test.py', wsPath]);
            let output = '';
            pythonProcess.stdout.on('data', (data) => {
                output += data.toString();
            });
            console.log(output);
            pythonProcess.stderr.on('data', (data) => {
                console.error(`stderr: ${data}`);
            });
            pythonProcess.on('close', (code) => {
                if (code !== 0) {
                    console.error(`child process exited with code ${code}`);
                }
                else {
                    console.log(`Python script output: ${output}`);
                    // ... further processing of the output ...
                }
            });
        }
    });
    context.subscriptions.push(disposable);
}
exports.activate = activate;
// This method is called when your extension is deactivated
function deactivate() { }
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map