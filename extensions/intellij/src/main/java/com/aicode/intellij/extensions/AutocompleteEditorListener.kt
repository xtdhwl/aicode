package com.aicode.intellij.extensions

import com.intellij.openapi.editor.event.EditorFactoryEvent
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.editor.event.EditorFactoryListener

class AutocompleteEditorListener: EditorFactoryListener {

//    private val logger = LoggerFactory.getLogger(AutocompleteEditorListener::class.java)
    private val logger = Logger.getInstance(AutocompleteEditorListener::class.java)

    override fun editorCreated(event: EditorFactoryEvent) {
        super.editorCreated(event)
        logger.info("Editor created: ${event.editor}")
    }

    override fun editorReleased(event: EditorFactoryEvent) {
        super.editorReleased(event)
        logger.info("Editor released: ${event.editor}")
    }
}