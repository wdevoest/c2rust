// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
const spawn = require("child_process").spawn;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

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
			const wsPath = vscode.workspace.workspaceFolders![0].uri.fsPath; // gets the path of the first workspace folder
			
			const filePath = vscode.Uri.file(wsPath + '/hello/world.md');
			vscode.window.showInformationMessage(filePath.toString());
			wsedit.createFile(filePath, { ignoreIfExists: true });
			vscode.workspace.applyEdit(wsedit);
			vscode.window.showInformationMessage('Created a new file: hello/world.md');
		
			const pythonProcess = await spawn('python3', ['Users/alexs/documents/mhacks24/c2rust/test.py', wsPath]);

			let output = '';
			pythonProcess.stdout.on('data', (data: Buffer) => {
				output += data.toString();
			});
			console.log(output);

			pythonProcess.stderr.on('data', (data: Buffer) => {
				console.error(`stderr: ${data}`);
			});

			pythonProcess.on('close', (code: Number) => {
				if (code !== 0) {
					console.error(`child process exited with code ${code}`);
				} else {
					console.log(`Python script output: ${output}`);
					// ... further processing of the output ...
				}
			});
		}
		
		
	});

	context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {}
