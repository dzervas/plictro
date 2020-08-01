use rubble::att::{AttUuid, Attribute, AttributeProvider, Handle, HandleRange};
use rubble::utils::HexSlice;
use rubble::uuid::Uuid16;
use rubble::Error;
use core::cmp;

pub struct HIDServiceAttrs<'a> {
	attributes: [Attribute<'a>; 5],
}

// HID Service (UUID: 1812)
// HID Characteristic (UUID: 7772E5DB-3868-4112-A1A9-F2669D106BF3)

// Boot Keyboard report length: 1 (modifier) + 1 (reserved) + 6 (keycode)
// Consumer control report length: 2 (usage value)

impl<'a> HIDServiceAttrs<'a> {
	pub fn new() -> Self {
		Self {
			attributes: [
				Attribute {
					att_type: Uuid16(0x2800).into(), // "Primary Service"
					handle: Handle::from_raw(0x0001),
					value: HexSlice(&[0x12, 0x18]), // "HID Service" = 0x1812
				},
				// HID Report Descriptor
				Attribute {
					att_type: Uuid16(0x2803).into(), // "Characteristic"
					handle: Handle::from_raw(0x0002),
					value: HexSlice(&[
						0x02 | 0x04, // 1 byte properties: READ = 0x02, WRITE_CMD = 0x04
						0x03, 0x00, // 2 bytes handle = 0x0003
						0x4C, 0x2A // 2 bytes UUID = 0x2A4C (HID Control Point)
// 0 is boot mode, 1 is report mode
					]),
				},
				// Here we should send the report map

				// Boot Keyboard Input Report - 8 bytes
				Attribute {
					att_type: AttUuid::Uuid16(Uuid16(0x2803)), // "Characteristic"
					handle: Handle::from_raw(0x0003),
					value: HexSlice(&[
						0x02 | 0x10, // 1 byte properties: READ = 0x02, NOTIFICATION = 0x10
						0x04, 0x00, // 2 bytes handle = 0x0004
						0x22, 0x2A // 2 bytes UUID = 0x2A22 (Boot Keyboard Input Report)
					]),
				},

				// Boot Keyboard Output Report - 1 byte (caps lock n shit)
				Attribute {
					att_type: AttUuid::Uuid16(Uuid16(0x2803)), // "Characteristic"
					handle: Handle::from_raw(0x0004),
					value: HexSlice(&[
						0x02 | 0x04 | 0x08, // 1 byte properties: READ = 0x02, WRITE_CMD = 0x04, WRITE_REQ = 0x08
						0x05, 0x00, // 2 bytes handle = 0x0005
						0x32, 0x2A // 2 bytes UUID = 0x2A32 (Boot Keyboard Output Report)
					]),
				},

                // Characteristic value (Keyboard output)
                Attribute {
                    att_type: AttUuid::Uuid16(Uuid16(0x2A32)), // "Boot Keyboard Output Report"
                    handle: Handle::from_raw(0x0003),
                    value: HexSlice(&[0u8]),
                },
			],
		}
	}
}

impl<'a> AttributeProvider for HIDServiceAttrs<'a> {
	fn for_attrs_in_range(
		&mut self,
		range: HandleRange,
		mut f: impl FnMut(&Self, Attribute<'_>) -> Result<(), Error>,
	) -> Result<(), Error> {
		let count = self.attributes.len();
		let start = usize::from(range.start().as_u16() - 1); // handles start at 1, not 0
		let end = usize::from(range.end().as_u16() - 1);

		let attrs = if start >= count {
			&[]
		} else {
			let end = cmp::min(count - 1, end);
			&self.attributes[start..=end]
		};

		for attr in attrs {
			f(
				self,
				Attribute {
					att_type: attr.att_type,
					handle: attr.handle,
					value: attr.value,
				},
			)?;
		}
		Ok(())
	}

	fn is_grouping_attr(&self, uuid: AttUuid) -> bool {
		uuid == Uuid16(0x2800) // FIXME not characteristics?
	}

	fn group_end(&self, handle: Handle) -> Option<&Attribute<'_>> {
		match handle.as_u16() {
			0x0001 => Some(&self.attributes[3]),
			0x0002 => Some(&self.attributes[3]),
			_ => None,
		}
	}
}
