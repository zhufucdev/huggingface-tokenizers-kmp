package tokenizers

expect class Encoding {
    val tokens: List<String>
    val ids: List<UInt>
    val sequenceIds: List<ULong?>
    val attentionMask: List<UInt>
    val size: Int

    override fun equals(other: Any?): Boolean
    override fun hashCode(): Int
}
