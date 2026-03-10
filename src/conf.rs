use crate::theme::Theme;

#[derive(Clone)]
pub struct Conf {
    pub theme: Theme,
    pub banner: String,
}

impl Conf {
    pub fn get() -> Self {
        Self {
            theme: Theme::new("colors"),
            banner: r#"
                                    ,       
                                    Et      
         .        .,                E#t     
        ;W       ,Wt                E##t    
       f#E      i#D.  GEEEEEEEL     E#W#t   
     .E#f      f#f    ,;;L#K;;.     E#tfL.  
    iWW;     .D#i        t#E        E#t     
   L##Lffi  :KW,         t#E     ,ffW#Dffj. 
  tLLG##L   t#f          t#E      ;LW#ELLLf.
    ,W#i     ;#G         t#E        E#t     
   j#E.       :KE.       t#E        E#t     
 .D#j          .DW:      t#E        E#t     
,WK,             L#,     t#E        E#t     
EG.               jt      fE        E#t     
,                          :        ;#t     
                                     :;    "#
                .to_string(),
        }
    }
}
