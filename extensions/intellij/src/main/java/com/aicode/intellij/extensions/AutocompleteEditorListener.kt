package com.aicode.intellij.extensions

import com.intellij.openapi.editor.event.CaretEvent
import com.intellij.openapi.editor.event.CaretListener
import org.slf4j.LoggerFactory
import com.intellij.openapi.diagnostic.Logger

class AutocompleteEditorListener: CaretListener {

//    private val logger = LoggerFactory.getLogger(AutocompleteEditorListener::class.java)
    private val logger = Logger.getInstance(AutocompleteEditorListener::class.java)

    override fun caretPositionChanged(event: CaretEvent) {
        logger.info("Caret position changed to: ${'$'}{event.caret.position}")
        super.caretPositionChanged(event)
    }

    override fun caretAdded(event: CaretEvent) {
        logger.info("Caret added at: ${'$'}{event.caret.position}")



        super.caretAdded(event)
    }

    override fun caretRemoved(event: CaretEvent) {
        logger.info("Caret removed from position: ${'$'}{event.caret.position}")

        super.caretRemoved(event)
    }
}