

package com.aicode.intellij.extensions.demo

import com.intellij.codeInsight.editorActions.TypedHandlerDelegate
import com.intellij.openapi.command.WriteCommandAction
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.Project
import com.intellij.psi.PsiFile
import org.jetbrains.annotations.NotNull
import com.intellij.openapi.diagnostic.Logger




class MyTypedHandler : TypedHandlerDelegate() {

    private val log = Logger.getInstance(MyTypedHandler::class.java)

    override fun charTyped(c: Char, project: Project, editor: Editor, file: PsiFile): Result {
        // 打印日志
        log.debug("Inserted 'editor_basics' at the beginning of the document.")


                // Get the document and project
//        val document: Document = editor.getDocument()
//        // Construct the runnable to substitute the string at offset 0 in the document
//        val runnable = Runnable { document.insertString(0, "editor_basics\n") }
//        // Make the document change in the context of a write action.
//        WriteCommandAction.runWriteCommandAction(project, runnable)

        return Result.STOP
    }
}