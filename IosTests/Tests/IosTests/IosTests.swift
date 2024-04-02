import XCTest
import ImageProvider
@testable import Binding

final class IosTests: XCTestCase {
    func testExample() throws {
        let status = ImageProvider.imagePath.withCString { test_read_from_png_file_to_memory($0) }
        XCTAssertEqual(status, 1, "Wrong png operation status: '\(status)', success is '1'")
    }
}
