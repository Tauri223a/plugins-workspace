// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package app.tauri.clipboard

import android.R.attr.value
import android.app.Activity
import android.content.ClipData
import android.content.ClipDescription
import android.content.ClipboardManager
import android.content.Context
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.annotation.JsonDeserialize

@JsonDeserialize(using = WriteOptionsDeserializer::class)
sealed class WriteOptions {
  class PlainText(val text: String, val label: String?): WriteOptions()
}

internal class WriteOptionsDeserializer: JsonDeserializer<WriteOptions>() {
  override fun deserialize(
    jsonParser: JsonParser,
    deserializationContext: DeserializationContext
  ): WriteOptions {
    val node: JsonNode = jsonParser.codec.readTree(jsonParser)
    node.get("plainText")?.let {
      return jsonParser.codec.treeToValue(it, WriteOptions.PlainText::class.java)
    }
    throw Error("unknown write options $node")
  }
}

@TauriPlugin
class ClipboardPlugin(private val activity: Activity) : Plugin(activity) {
  private val manager: ClipboardManager =
    activity.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager

  @Command
  @Suppress("MoveVariableDeclarationIntoWhen")
  fun write(invoke: Invoke) {
    val args = invoke.parseArgs(WriteOptions::class.java)

    val clipData = when (args) {
      is WriteOptions.PlainText -> {
        ClipData.newPlainText(args.label, args.text)
      }

      else -> {
        invoke.reject("Unimplemented $args")
        return
      }
    }

    manager.setPrimaryClip(clipData)

    invoke.resolve()
  }

  @Command
  fun read(invoke: Invoke) {
    val (kind, options) = if (manager.hasPrimaryClip()) {
      if (manager.primaryClipDescription?.hasMimeType(ClipDescription.MIMETYPE_TEXT_PLAIN) == true) {
        val item: ClipData.Item = manager.primaryClip!!.getItemAt(0)
        Pair("PlainText", item.text)
      } else {
        // TODO
        invoke.reject("Clipboard content reader not implemented")
        return
      }
    } else {
      invoke.reject("Clipboard is empty")
        return
    }

    val response = JSObject()
    response.put("kind", kind)
    response.put("options", options)
    invoke.resolve(response)
  }
}
