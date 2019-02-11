use afl::fuzz;
use libtorset::MicroDescriptor;

fn main()
{
	fuzz!( |data: &[u8]|
	{
		if let Ok(s) = std::str::from_utf8( data )
		{
			let _ = MicroDescriptor::new( &s );
		}

	});
}
