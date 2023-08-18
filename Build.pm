class Build {
    method build($dist-path) {
        mkdir 'resources';
        mkdir 'resources/dan';
        move 'dan/src/*', 'resources/dan/src';
        
        chdir 'resources/dan/src';        
        warn ' Building Rust Polars library (may take a few minutes).';
        my $proc = Proc::Async.new: <cargo build>;
        $proc.bind-stdout($*ERR);
        my $promise = $proc.start;
        await $promise;
        


        #`[
        mkdir 'resources/apply';
        mkdir 'resources/apply/src';
        move 'dan/src/apply-template.rs', 'resources/apply/src/apply-template.rs';
        move 'dan/target/debug/deps/*',   'resources/apply/target/debug/deps/';
        #]
        
        warn 'Build successful';
        
        exit 0
    }
}
