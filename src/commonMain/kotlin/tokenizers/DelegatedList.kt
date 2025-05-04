package tokenizers

abstract class DelegatedList<T>(override val size: Int) : List<T> {
    override fun get(index: Int): T {
        if (index >= size) throw IndexOutOfBoundsException()
        return getSafe(index)
    }

    abstract fun getSafe(index: Int): T

    override fun isEmpty(): Boolean {
        return size <= 0
    }

    override fun contains(element: T): Boolean =
        (0 until size).any { get(it) == element }

    override fun iterator(): Iterator<T> = iterator {
        var index = 0
        while (index < size) {
            yield(get(index++))
        }
    }

    override fun containsAll(elements: Collection<T>): Boolean =
        elements.all { contains(it) }

    override fun indexOf(element: T): Int =
        (0 until size).firstOrNull { get(it) == element } ?: -1

    override fun lastIndexOf(element: T): Int =
        (0 until size).lastOrNull { get(it) == element } ?: -1

    inner class DelegatedListIterator(private var currIdx: Int = 0) : ListIterator<T> {
        override fun next(): T = get(currIdx++)

        override fun hasNext(): Boolean = currIdx < size

        override fun hasPrevious(): Boolean = currIdx > 0

        override fun previous(): T = get(--currIdx)

        override fun nextIndex(): Int = currIdx + 1

        override fun previousIndex(): Int = currIdx - 1
    }

    override fun listIterator(): ListIterator<T> = DelegatedListIterator()

    override fun listIterator(index: Int): ListIterator<T> = DelegatedListIterator(index)

    inner class OffsetDelegatedList(private val offset: Int): DelegatedList<T>(size - offset) {
        override fun getSafe(index: Int): T = this@DelegatedList[index + offset]
    }

    override fun subList(fromIndex: Int, toIndex: Int): List<T> = OffsetDelegatedList(fromIndex)
}