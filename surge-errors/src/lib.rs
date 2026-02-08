use error_tree::*;

error_tree!{

    #[derive(Debug)]
    pub enum SurgeError {
        AlignmentError(AlignmentError),
    }

    #[derive(Debug)]
    pub enum ConvertError {
        Default,
    }

    #[derive(Debug)]
    pub enum AlignmentError {

        #[display("AlignmentError: src_ptr={idx}, required_align={required_align}")]
        SrcPtr { idx: usize, required_align: usize },

        #[display("AlignmentError: dst_ptr={idx}, required_align={required_align}")]
        DstPtr { idx: usize, required_align: usize },
    }
}
